use std::fmt;
use std::path::Path;

use glium::texture::{MipmapsOption, UncompressedFloatFormat};
use nifti::{IntoNdArray, NiftiObject};
use tracing::{debug, info};

pub struct Image3D {
    pub data: Vec<f32>,
    pub shape: (u32, u32, u32),
    pub spacing: (f32, f32, f32),
    pub format: Option<UncompressedFloatFormat>,
    pub mipmaps: MipmapsOption,
    pub is_mask: bool,
}

impl fmt::Debug for Image3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image3D")
            .field("shape", &self.shape)
            .field("spacing", &self.spacing)
            .field("format", &self.format)
            .field("mipmaps", &self.mipmaps)
            .field("is_mask", &self.is_mask)
            .finish()
    }
}

pub fn load_image3d(data_path: &Path) -> Image3D {
    info!("Loading image from {:?}", data_path);
    let stem = data_path.file_name().unwrap().to_str().unwrap();
    if &stem[stem.len() - 6..] == "nii.gz" || &stem[stem.len() - 6..] == "hdr.gz" {
        debug!("Loading nifti file");
        let obj = nifti::ReaderOptions::new().read_file(data_path).unwrap();
        debug!("Loaded nifti file");
        let header = obj.header();
        let dim = header.dim;
        let spacing = header.pixdim;

        match header.datatype {
            4 => {
                // i16
                let data = obj
                    .into_volume()
                    .into_ndarray::<i16>()
                    .unwrap()
                    .map(|x: &i16| *x as f32)
                    .into_raw_vec();

                Image3D {
                    data,
                    shape: (dim[1] as u32, dim[2] as u32, dim[3] as u32),
                    spacing: (spacing[1], spacing[2], spacing[3]),
                    format: Some(UncompressedFloatFormat::F32),
                    mipmaps: MipmapsOption::NoMipmap,
                    is_mask: false,
                }
            }
            64 => {
                // double
                let data = obj
                    .into_volume()
                    .into_ndarray::<f64>()
                    .unwrap()
                    .map(|x: &f64| *x as f32)
                    .into_raw_vec();
                Image3D {
                    data,
                    shape: (dim[1] as u32, dim[2] as u32, dim[3] as u32),
                    spacing: (spacing[1], spacing[2], spacing[3]),
                    format: Some(UncompressedFloatFormat::F32),
                    mipmaps: MipmapsOption::NoMipmap,
                    is_mask: true,
                }
            }
            _ => panic!("Unsupported data type : {}", header.datatype),
        }
    } else {
        panic!("Unsupported file format");
    }
}

// use image;
// use image::{ImageBuffer, Rgba};
// use ndarray::prelude::*;
// use ndarray::{Array, ArrayD, IxDyn};

// pub fn load_image_slice(data_path: &Path) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
//     println!("Loading image from {:?}", data_path);
//     let stem = data_path.file_name().unwrap().to_str().unwrap();
//     if &stem[stem.len() - 6..] == "nii.gz" || &stem[stem.len() - 6..] == "hdr.gz" {
//         let obj = nifti::ReaderOptions::new().read_file(data_path).unwrap();
//         let header = obj.header();
//         let dim = header.dim;
//         let spacing = header.pixdim;
//         let endianness = header.endianness;
//         let data: ArrayD<i16> = obj.into_volume().into_ndarray().unwrap();

//         image::ImageBuffer::from_fn(dim[1] as u32, dim[2] as u32, |x, y| {
//             let pixel = data[[x as usize, y as usize, 100]] as f32;
//             let pixel = pixel as f32;
//             Rgba([pixel, pixel, pixel, 1.0f32])
//         })
//     } else {
//         panic!("Unsupported file format");
//     }
// }
