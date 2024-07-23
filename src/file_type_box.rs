use byteorder::{BigEndian, ReadBytesExt};

use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};

#[derive(Debug)]
pub struct FileTypeBox {
    pub major_brand: String,
    pub major_version: u32,
    pub compatible_brands: Vec<String>,
}

impl FileTypeBox {
    pub fn new(reader: &mut BufReader<File>, file_offset: u32, size: u32) -> FileTypeBox {
        reader
            // FIXME: u64??
            .seek(SeekFrom::Start((file_offset + 4) as u64)) // skip ftyp header
            .unwrap();

        let mut long_string: [u8; 4] = [0; 4];
        reader.read(&mut long_string).unwrap();
        let _ = String::from_utf8_lossy(&long_string).into_owned();

        let mut long_string: [u8; 4] = [0; 4];
        reader.read(&mut long_string).unwrap();
        let major_brand = String::from_utf8_lossy(&long_string).into_owned();

        let major_version = reader.read_u32::<BigEndian>().unwrap();

        // box_size minus (size, type, brand, version) which are all 4 bytes wide
        let mut compatible_brands: Vec<String> = vec![];
        let compatible_count = (size - (4 * 4)) / 4;
        for _ in 0..compatible_count {
            let mut long_string: [u8; 4] = [0; 4];
            reader.read(&mut long_string).unwrap();
            let brand = String::from_utf8_lossy(&long_string);
            compatible_brands.push(brand.to_string());
        }

        return FileTypeBox {
            major_brand,
            major_version,
            compatible_brands,
        };
    }
}
