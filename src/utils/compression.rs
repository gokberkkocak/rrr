use std::io;
use std::{fs, io::Cursor};
const COMPRESSION_LEVEL: i32 = 1;

pub fn _compress_file_to_file(source: &str) {
    let mut file = fs::File::open(source).unwrap();
    let mut encoder = {
        let target = fs::File::create(source.to_string() + super::ZST_SUFFIX).unwrap();
        zstd::Encoder::new(target, COMPRESSION_LEVEL).unwrap()
    };

    io::copy(&mut file, &mut encoder).unwrap();
    encoder.finish().unwrap();
}

pub fn compress_string_to_file(source: String, source_filename: &str) {
    let mut encoder = {
        let target = fs::File::create(
            source_filename
                .trim_end_matches(super::ZST_SUFFIX)
                .to_string()
                + super::ZST_SUFFIX,
        )
        .unwrap();
        zstd::Encoder::new(target, COMPRESSION_LEVEL).unwrap()
    };
    let mut source_cursor = Cursor::new(source);
    io::copy(&mut source_cursor, &mut encoder).unwrap();
    encoder.finish().unwrap();
}

pub fn _decompress_file_to_file(source: &str) {
    let mut decoder = {
        let file = fs::File::open(source).unwrap();
        zstd::Decoder::new(file).unwrap()
    };
    let mut target = fs::File::create(source.trim_end_matches(super::ZST_SUFFIX)).unwrap();
    io::copy(&mut decoder, &mut target).unwrap();
}

pub fn decompress_file_to_string(source: &str) -> String {
    let mut decoder = {
        let file = fs::File::open(source).unwrap();
        zstd::Decoder::new(file).unwrap()
    };
    let mut target_writer = vec![];
    io::copy(&mut decoder, &mut target_writer).unwrap();
    String::from_utf8(target_writer).unwrap()
}
