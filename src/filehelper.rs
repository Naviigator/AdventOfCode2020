use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_file(day: i32, dummy_data: bool, line_parser: &mut dyn FnMut(String)) {
    let mut sample_text = "";
    if dummy_data {    
        println!("WARNING, using dummy data! Do not use this as an answer!");
        sample_text = "_sample";
    }
    let sample_file_name = format!("data/day{}{}.txt", day, sample_text);
    let sample_file = File::open(sample_file_name).expect("unable to open file");
    let sample_reader = BufReader::new(sample_file);

    for line in sample_reader.lines() {
        line_parser(line.unwrap());
    }
}