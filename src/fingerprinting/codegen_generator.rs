use super::super::structures::fingerprint::Fingerprint;
use super::*;

pub fn generate_code(samples: &[f32], start_offset: i32) -> Result<Fingerprint, &'static str>
{
    if samples.len() < 100 {
        return Err("Not enough samples");
    }
    if samples.len() >= constants::MAX_SAMPLES {
        return Err("File is too big");
    }
    let whitened = whitening::compute(samples);
    let subbands = subband_analysis::compute(&whitened);
    let fingerprint = fingerprint_calculations::compute(subbands, start_offset);

    Ok(fingerprint)
}