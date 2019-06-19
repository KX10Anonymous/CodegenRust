extern crate byteorder;

use std::fs;
use std::io;
use byteorder::{LittleEndian, ReadBytesExt};

fn main() {
    let path = "raw16";
    let raw_file_size = fs::metadata(path).expect("Cannot open raw file").len() as usize;
    let raw_file = fs::File::open(path).expect("Cannot open raw file");
    let mut buffered_reader = io::BufReader::new(raw_file);

    let samples_size = raw_file_size / 2;
    let mut samples = Vec::with_capacity(samples_size);

    while let Ok(value) = buffered_reader.read_i16::<LittleEndian>() {
        let sample : f32 = (value as f32) / 32768.0;
        samples.push(sample);
    }

    assert_eq!(samples.len(), samples_size);
    assert_eq!(samples[0], 0.0);
    assert_eq!(samples[28], 0.000030517578);
    assert_eq!(samples[30], 0.000030517578);
}
