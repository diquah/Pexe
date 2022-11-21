use std::{fs, path::Path, io::BufWriter};
use serde::Serialize;
use tar::Builder;
use base91::slice_encode;
use lz4_flex::compress_prepend_size;
use rmp_serde::Serializer;

use crate::format_spec::{PexeConfig, PexeFileFormat};

pub fn build(dir: &String) {
    let project_file = fs::read_to_string(format!("{}/project.toml", dir))
        .expect("Error reading `project.toml` file.");
    let project_config: PexeConfig = toml::from_str(&project_file).unwrap();

    let tarball: Vec<u8> = vec![];
    let mut tar_builder = Builder::new(tarball);

    for f in fs::read_dir(dir).unwrap() {
        let file = f.unwrap().path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let file_metadata = file.metadata().unwrap();
        let relative_path = file.strip_prefix(Path::new(dir)).unwrap().to_str().unwrap();
        
        if file_metadata.is_dir() {
            if project_config.project.include.contains(&file_name) {
                tar_builder.append_dir_all(relative_path, file.to_str().unwrap());
            }
        } else if file_metadata.is_file() {
            if file.extension().unwrap() == "py" {
                tar_builder.append_file(relative_path, &mut fs::File::open(&file).unwrap());
            } else if project_config.project.include.contains(&file_name) {
                tar_builder.append_file(relative_path, &mut fs::File::open(&file).unwrap());
            }
        }
    }

    let result = tar_builder.into_inner().unwrap();
    let result_compressed = compress_prepend_size(&result);
    let result_b91_bin = slice_encode(&result_compressed);
    let result_b91 = String::from_utf8_lossy(&result_b91_bin);

    let pexe_package = PexeFileFormat {
        name: project_config.project.name.clone(),
        python_version: project_config.project.python_version,
        tarball: result_b91.to_string()
    };

    
    let pexe_path = Path::new(dir).join(format!("{}.pexe", project_config.project.name));
    let pexe = fs::File::create(&pexe_path).unwrap();
    let mut pexe_writer = BufWriter::new(pexe);

    let pexe_data = pexe_package.serialize(&mut Serializer::new(&mut pexe_writer)).unwrap();
}