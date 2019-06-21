extern crate codegen;
use codegen::calculate_for_raw_samples;

fn main()
{
    let result = calculate_for_raw_samples("raw16").expect("Cannot generate fingerprint");
    println!("{}", result.codes.len());
}