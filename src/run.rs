use std::{fs, io, path::Path};
use base91::slice_decode;
use lz4_flex::decompress_size_prepended;
use tar::Archive;

use crate::format_spec::{PexeConfig, PexeFileFormat};

pub fn run(file: &String) {
    let pexe_file = fs::File::open(file).unwrap();
    let pexe_file_reader = io::BufReader::new(pexe_file);

    let pexe_data: PexeFileFormat = rmp_serde::from_read(pexe_file_reader).unwrap();

    let tarball_raw = pexe_data.tarball.as_bytes();
    let tarball_decode91 = slice_decode(tarball_raw);
    let tarball_uncompressed = decompress_size_prepended(&tarball_decode91).unwrap();

    let mut tarball = Archive::new(io::Cursor::new(tarball_uncompressed));
    tarball.unpack(Path::new(file).parent().unwrap().join("unpack")).unwrap();
    
}