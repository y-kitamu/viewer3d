use std::fmt;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use glium::texture::{MipmapsOption, UncompressedFloatFormat};
use nifti::{IntoNdArray, NiftiObject};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Serialize, Deserialize)]
pub struct Image3D {
    #[serde(skip)]
    pub data: Vec<f32>,
    pub shape: (u32, u32, u32),
    pub spacing: (f32, f32, f32),
    #[serde(skip)]
    pub format: Option<UncompressedFloatFormat>,
    #[serde(skip)]
    pub mipmaps: Option<MipmapsOption>,
    pub is_mask: bool,
}

impl Image3D {
    pub fn serialize(&self, path: &Path) {
        // voxelデータはrawファイルに、それ以外の情報はjsonファイルに保存する
        let header_path = path.with_extension("json");
        let raw_path = path.with_extension("raw");
        // write data
        let mut writer = BufWriter::new(std::fs::File::create(raw_path).unwrap());
        for val in &self.data {
            writer.write_all(&val.to_ne_bytes()).unwrap();
        }
        // write header
        let json = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(header_path, json).unwrap();
        info!("Serialized image to {:?}", path);
    }

    pub fn deserialize(path: &Path) -> Image3D {
        let header_path = path.with_extension("json");
        let raw_path = path.with_extension("raw");
        // read header
        let json = std::fs::read_to_string(header_path).unwrap();
        let mut image: Image3D = serde_json::from_str(&json).unwrap();
        image.format = Some(UncompressedFloatFormat::F32);
        image.mipmaps = Some(MipmapsOption::NoMipmap);
        // read data
        let mut data = Vec::new();
        let mut reader = BufReader::new(std::fs::File::open(raw_path).unwrap());
        let mut buffer = [0; 4];
        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }
            data.push(f32::from_ne_bytes(buffer));
        }
        info!("Deserialized image from {:?}", path);
        image.data = data;
        image
    }
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
    if &stem[stem.len() - 4..] == ".nii"
        || &stem[stem.len() - 6..] == "nii.gz"
        || &stem[stem.len() - 6..] == "hdr.gz"
    {
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
                    mipmaps: Some(MipmapsOption::NoMipmap),
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
                    mipmaps: Some(MipmapsOption::NoMipmap),
                    is_mask: true,
                }
            }
            _ => panic!("Unsupported data type : {}", header.datatype),
        }
    } else {
        panic!("Unsupported file format");
    }
}
