extern crate byteorder;

mod fingerprinting;
mod input;
mod structures;

use input::*;
use fingerprinting::codegen_generator;
use std::path::Path;
use crate::structures::fingerprint::Fingerprint;

pub fn calculate_for_raw_samples<P>(path: P) -> Result<Fingerprint, String>
    where P : AsRef<Path>
{
    let samples = samples_reader::get_samples(path).map_err(|err| err.to_string())?;
    let result = codegen_generator::generate_code(&samples, 0)?;
    Ok(result)
}

pub fn calculate_for_audio_file<P>(path: P) -> Result<Fingerprint, String>
    where P : AsRef<Path>
{
    // TODO : call ffmpeg for raw samples output
    Ok(Fingerprint::new(vec![]))
}

#[test]
fn test_main() {
    let samples = samples_reader::get_samples("raw16").expect("Cannot read samples file");
    assert_eq!(samples.len(), 3103688);

    let result = codegen_generator::generate_code(&samples, 0).expect("Cannot generate codes");
    assert_eq!(result.codes.len(), 6948);

    let test_data_vector = test_data::tests::get_test_data("test_data").expect("Cannot read test data");
    assert_eq!(test_data_vector.len(), 6948);

    // compare result to test_data
    for i in 0..result.codes.len() {
        assert_eq!(result.codes[i], test_data_vector[i]);
    }
}