use std::fs;
use std::io;
use std::path::Path;
use byteorder::{LittleEndian, ReadBytesExt};

pub fn get_samples<P>(path: P) -> io::Result<Vec<f32>>
    where P : AsRef<Path>
{
    let file = fs::File::open(&path)?;
    let file_size = file.metadata()?.len() as usize;
    let mut buffered_reader = io::BufReader::new(file);

    let mut samples = Vec::with_capacity(file_size / 2);

    while let Ok(value) = buffered_reader.read_i16::<LittleEndian>() {
        let sample : f32 = (value as f32) / 32768.0;
        samples.push(sample);
    }
    Ok(samples)
}

#[test]
fn proper_test_data()
{
    let samples = get_samples("raw16").unwrap();

    assert_eq!(samples.len(), 3103688);
    assert_eq!(samples[0], 0.0);
    assert_eq!(samples[28], 0.000030517578);
    assert_eq!(samples[30], 0.000030517578);
}