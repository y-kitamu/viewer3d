use std::path::Path;

use nifti::NiftiObject;

pub struct Image3D {
    pub data: Vec<u8>,
    pub shape: (u32, u32, u32),
    pub spacing: (f32, f32, f32),
}

fn reverse_endianness(data: &Vec<u8>, step: usize) -> Vec<u8> {
    let mut reversed = Vec::new();
    for i in (0..data.len()).step_by(step) {
        reversed.push(data[i + 3]);
        reversed.push(data[i + 2]);
        reversed.push(data[i + 1]);
        reversed.push(data[i]);
    }
    reversed
}

pub fn load_image3d(data_path: &Path) -> Image3D {
    if data_path.ends_with("nii.gz") || data_path.ends_with("hdr.gz") {
        let obj = nifti::ReaderOptions::new().read_file(data_path).unwrap();
        let header = obj.header();
        let dim = header.dim;
        let spacing = header.pixdim;
        let endianness = header.endianness;
        let mut data = obj.into_volume().into_raw_data();

        if endianness == nifti::Endianness::Little {
            data = reverse_endianness(&data, 2);
        }

        println!(
            "dim : {:?}, spacing : {:?}, endianness : {:?}",
            &dim[1..4],
            &spacing[1..4],
            endianness
        );

        Image3D {
            data,
            shape: (dim[1] as u32, dim[2] as u32, dim[3] as u32),
            spacing: (spacing[1], spacing[2], spacing[3]),
        }
    } else {
        panic!("Unsupported file format");
    }
}
