extern crate byteorder;
extern crate ole32;

mod fingerprinting;
mod input;
mod structures;

use input::*;
use fingerprinting::codegen_generator;
use std::path::Path;
use crate::structures::fingerprint::Fingerprint;
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::str::from_utf8;
use crate::structures::code_segment::CodeSegment;
use std::mem::size_of;
use ole32::CoTaskMemAlloc;

/// this function is for use in external languages
/// C# example:
/// [DllImport("codegen.dll", CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
/// unsafe static extern uint calculate_for_raw_samples_c([In] byte[] buffer, [In] uint bufferLength, [Out] CodeSegment** array);
///
/// calling:
/// var bytes = Encoding.UTF8.GetBytes($@"{basePath}raw16");
/// CodeSegment* segments;
/// var length = calculate_for_raw_samples_c(bytes, (uint)bytes.Length, &segments);
#[no_mangle]
pub unsafe extern "C" fn calculate_for_raw_samples_c(path: *const u8, path_length: u32, array_ptr: *mut *mut CodeSegment) -> u32
{
    let slice = from_raw_parts(path, path_length as usize);
    let path_encoded = match from_utf8(slice) {
        Ok(v) => v,
        Err(_) => {
            println!("Error during parsing path");
            return 0
        },
    };

    match calculate_for_raw_samples(path_encoded) {
        Ok(fingerprint) => {
            let length = fingerprint.codes.len();
            let bytes_to_alloc = length * size_of::<CodeSegment>();
            *array_ptr = CoTaskMemAlloc(bytes_to_alloc as u64) as *mut CodeSegment;

            let array = from_raw_parts_mut(*array_ptr, length);
            for (index, code_segment) in array.iter_mut().enumerate() {
                code_segment.code = fingerprint.codes[index].code;
                code_segment.time = fingerprint.codes[index].time;
            }
            return length as u32
        },
        Err(e) => {
            println!("Error during fingerprinting: {}", e);
            return 0
        },
    }
}

pub fn calculate_for_raw_samples<P>(path: P) -> Result<Fingerprint, String>
    where P : AsRef<Path>
{
    let samples = samples_reader::get_samples(path).map_err(|err| err.to_string())?;
    let result = codegen_generator::generate_code(&samples, 0)?;
    Ok(result)
}

pub fn calculate_for_audio_file<P>(_path: P) -> Result<Fingerprint, String>
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