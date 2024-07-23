use parser::IsoParser;
use std::{fs::File, io::BufReader};

mod file_type_box;
mod iso_box;
mod parser;

fn main() {
    let file = File::open("bbb_sunflower_1080p_30fps_normal.mp4").expect("Unable to open file");
    // let file = File::open("file_example_MOV_480_700kB.mov").expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let iso = IsoParser::new(&mut reader).unwrap();

    let boxes = iso.get_boxes(&mut reader).unwrap();
    println!("boxes: {boxes:#?}");

    let first_box = boxes.get(0).unwrap().clone();
    let first_box_data = first_box.get_data(&mut reader);
    println!("first_box data: {first_box_data:#?}");

    // Verify that there are no issues with making the calls again
    let boxes = iso.get_boxes(&mut reader).unwrap();
    println!("boxes: {boxes:?}");
}
