use base91::slice_encode;
use lz4_flex::compress_prepend_size;
use rmp_serde::Serializer;
use serde::Serialize;
use std::{fs, io::BufWriter, path::Path};
use tar::Builder;

use crate::format_spec::{PexeConfig, PexeFileFormat, PEXE_VERSION};

pub fn build(dir: &String) {
    let project_file = fs::read_to_string(format!("{}/project.toml", dir))
        .expect("Error reading `project.toml` file.");
    let project_config: PexeConfig = toml::from_str(&project_file).unwrap();

    let mut includes = project_config.project.includes.unwrap_or(vec![]);

    let tarball: Vec<u8> = vec![];
    let mut tar_builder = Builder::new(tarball);

    for f in fs::read_dir(dir).unwrap() {
        let file = f.unwrap().path();
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let file_metadata = file.metadata().unwrap();
        let relative_path = file.strip_prefix(Path::new(dir)).unwrap().to_str().unwrap();

        if file_metadata.is_dir() {
            if includes.contains(&file_name) {
                let _ = tar_builder.append_dir_all(relative_path, file.to_str().unwrap());
                includes.remove(includes.iter().position(|x| *x == file_name).unwrap());
            }
        } else if file_metadata.is_file() {
            if file.extension().unwrap() == "py" {
                let _ = tar_builder.append_file(relative_path, &mut fs::File::open(&file).unwrap());
            } else if includes.contains(&file_name) {
                let _ = tar_builder.append_file(relative_path, &mut fs::File::open(&file).unwrap());
                includes.remove(includes.iter().position(|x| *x == file_name).unwrap());
            }
        }
    }

    if includes.len() > 0 {
        println!(
            "WARNING: FOLLOWING FILES IN INCLUDE NOT FOUND IN PROJECT FOLDER: {:?}",
            includes
        );
    }

    let result = tar_builder.into_inner().unwrap();
    let result_compressed = compress_prepend_size(&result);
    let result_b91_bin = slice_encode(&result_compressed);
    let result_b91 = String::from_utf8_lossy(&result_b91_bin);

    let pexe_package = PexeFileFormat {
        pexe_version: PEXE_VERSION.to_string(),

        name: project_config.project.name.clone(),
        python_version: project_config.project.python_version,
        tarball: result_b91.to_string(),
    };

    let pexe_path = Path::new(dir).join(format!("{}.pexe", project_config.project.name));
    let pexe = fs::File::create(&pexe_path).unwrap();
    let mut pexe_writer = BufWriter::new(pexe);

    let _pexe_data = pexe_package
        .serialize(&mut Serializer::new(&mut pexe_writer))
        .unwrap();
}
