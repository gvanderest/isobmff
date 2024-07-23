use crate::file_type_box::FileTypeBox;
use std::{fs::File, io::BufReader};

#[derive(Debug, Clone)]
pub struct IsoBox {
    size: u32,
    data_size: u32,
    file_box_start_offset: u32,

    raw_type: String,
    r#type: IsoBoxType,
}

impl<'a> IsoBox {
    pub fn new(raw_type: String, size: u32, file_box_start_offset: u32) -> IsoBox {
        let r#type = match raw_type.as_str() {
            "moov" => IsoBoxType::Movie,
            "mvhd" => IsoBoxType::MovieHeader,
            "mdat" => IsoBoxType::MediaData,
            "free" => IsoBoxType::Free,
            "ftyp" => IsoBoxType::FileType,
            _ => IsoBoxType::Unknown,
        };

        return IsoBox {
            r#type,
            size,
            data_size: size,
            file_box_start_offset,
            raw_type,
        };
    }

    pub fn get_data(self, reader: &mut BufReader<File>) -> IsoBoxData {
        match self.r#type {
            IsoBoxType::FileType => {
                return IsoBoxData::FileType(FileTypeBox::new(
                    reader,
                    self.file_box_start_offset,
                    self.data_size,
                ));
            }
            _ => {
                let box_type = self.r#type.clone();
                panic!("Unhandled box type={box_type:?}");
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IsoBoxType {
    Unknown,
    FileType,
    Free,
    Movie,
    MovieHeader,
    MediaData,
}

#[derive(Debug)]
pub enum IsoBoxData {
    FileType(FileTypeBox),
}
