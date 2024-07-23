use crate::iso_box;
use byteorder::{BigEndian, ReadBytesExt};
use iso_box::IsoBox;
use std::io::{BufRead, Read, Seek, SeekFrom};
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone)]
pub struct IsoParser {
    detected_file_format: String,
}

impl IsoParser {
    // FIXME: Replace File with an `impl Read`?
    pub fn new(reader: &mut BufReader<File>) -> Result<IsoParser, String> {
        // FIXME: Learn how to format one of these new() functions properly
        let parser = IsoParser {
            detected_file_format: "unknown".to_string(),
            // boxes: vec![],
            // FIXME: Learn about Default
        };

        // parser.init(reader)?;

        Result::Ok(parser)
    }

    // fn init(&mut self, reader: &mut BufReader<File>) -> Result<(), String> {
    //     // let boxes = self.get_boxes(reader)?;

    //     // FIXME: Make this get the ftyp box explicitly
    //     // let first_box = boxes.get(0).unwrap().clone();
    //     // let data = first_box.get_data(reader);
    //     // println!("FIRST BOX: {data:?}");

    //     Result::Ok(())
    // }

    // FIXME: Make this do a result?
    pub fn get_boxes(&self, reader: &mut BufReader<File>) -> Result<Vec<IsoBox>, String> {
        let mut boxes = vec![];

        reader.seek(SeekFrom::Start(0)).unwrap();

        let mut file_box_start_offset: u32 = 0;

        loop {
            let buffer = reader.fill_buf().unwrap();
            if buffer.is_empty() {
                break;
            }

            let size = reader.read_u32::<BigEndian>().unwrap();

            // FIXME: Make a read_long_string function to get these easier
            let mut long_string: [u8; 4] = [0; 4];
            reader.read(&mut long_string).unwrap();
            let raw_box_type = String::from_utf8_lossy(&long_string).into_owned();

            // println!("size={size}, raw_box_type={raw_box_type}");

            boxes.push(IsoBox::new(raw_box_type, size, file_box_start_offset));

            // Seek forward for the entire box, minus the data we've already read (size and type)
            reader
                // FIXME: u64 and i64??
                .seek(SeekFrom::Current(size as i64))
                .unwrap();
            file_box_start_offset += size;
        }

        Result::Ok(boxes)
    }

    // pub fn get_boxes(&'a self) -> Vec<IsoBox> {
    //     return self.boxes.clone();
    // }

    pub fn get_detected_file_format(&self) -> String {
        return self.detected_file_format.clone();
    }
}
