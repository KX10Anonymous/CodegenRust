use super::super::structures::fingerprint::Fingerprint;
use super::super::structures::matrix::Matrix;
use std::f32::consts::PI;

pub fn compute(matrix: Matrix<f32>, offset: i32) -> Fingerprint
{
    let mut hash_material = [0_u8; 5];
    let mut onset_counter_for_band = [0; SUBBANDS];
    let mut p: Matrix<i32> = Matrix::new(2, 6);
    let mut out_matrix = adaptive_onsets(&mut onset_counter_for_band, matrix);

    // TODO : return real result
    Fingerprint::new(vec![])
}

fn adaptive_onsets(onset_counter_for_band: &mut [i32], e: Matrix<f32>) -> (Matrix<i32>, i32)
{
    let mut bands: i32;
    let mut frames: i32;

    let mut h = [0_f32; SUBBANDS];
    let mut taus = [0_f32; SUBBANDS];
    let mut n = [0_f32; SUBBANDS];
    let mut y0 = [0_f32; SUBBANDS];
    let mut ham = [0_f32; 8];

    let mut contact = [0_i32; SUBBANDS];
    let mut lcontact = [0_i32; SUBBANDS];
    let mut tsince = [0_i32; SUBBANDS];

    let mut onset_counter = 0;

    // Take successive stretches of 8 subband samples and sum their energy under a hann window, then hop by 4 samples (50% window overlap).
    for i in 0..NSM {
        ham[i as usize] = 0.5 - 0.5 * ((2.0 * PI / (NSM - 1) as f32) * i as f32).cos();
    }

    let nc = ((e.cols as f32 / HOP as f32).floor() - ((NSM as f32 / HOP as f32).floor() - 1.0)) as usize;
    let out_matrix: Matrix<i32> = Matrix::new(SUBBANDS, nc);
    let eb: Matrix<f32> = Matrix::new(nc, 8);

    // TODO : FingerprintCalculations l: 124

    (out_matrix, onset_counter)
}

const SUBBANDS: usize = 8;
const QUANTIZE_DT_S: f32 = 256.0 / 11025.0;
const QUANTIZE_A_S: f32 = 256.0 / 11025.0;
const HASH_BITMASK: u32 = 0x000fffff;
const TTARG: i32 = 345;
const DEADTIME: i32 = 128;
const BN: [f32; 3] = [0.1883, 0.4230, 0.3392];
const NBN:i32 = 3;
const A1:f32 = 0.98;
const OVERFACT:f32 = 1.1;
const HOP:f32 = 4.0;
const NSM:i32 = 8;