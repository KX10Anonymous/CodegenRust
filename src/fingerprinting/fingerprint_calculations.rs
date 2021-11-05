use super::super::structures::fingerprint::Fingerprint;
use super::super::structures::matrix::Matrix;
use super::super::structures::code_segment::CodeSegment;
use std::f32::consts::PI;
use super::constants;
use super::murmurhash2;
use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;


pub fn compute(matrix: Matrix<f32>, offset: i32) -> Fingerprint
{
    let mut hash_material = [0_u8; 6];
    let mut onset_counter_for_band = [0; SUBBANDS];
    let mut p: Matrix<i32> = Matrix::new(2, 6);
    let (out_matrix, onset_count) = adaptive_onsets(&mut onset_counter_for_band, matrix);

    let mut codes = Vec::with_capacity(onset_count * 6);

    for band in 0..SUBBANDS {
        if onset_counter_for_band[band] <= 2 { continue; }

        for onset in 0..onset_counter_for_band[band] - 2 {
            // What time was this onset at?
            let time_for_onset_ms_quantized = quantized_time_for_frame_absolute(out_matrix[[band, onset]], offset);

            for i in 0..6 {
                p[[0, i]] = 0;
                p[[1, i]] = 0;
            }
            let mut nhashes = 6;

            if onset == onset_counter_for_band[band] - 4
            {
                nhashes = 3;
            }
            if onset == onset_counter_for_band[band] - 3
            {
                nhashes = 1;
            }

            p[[0, 0]] = out_matrix[[band, onset + 1]] - out_matrix[[band, onset]];
            p[[1, 0]] = out_matrix[[band, onset + 2]] - out_matrix[[band, onset + 1]];

            if nhashes > 1
            {
                p[[0, 1]] = out_matrix[[band, onset + 1]] - out_matrix[[band, onset]];
                p[[1, 1]] = out_matrix[[band, onset + 3]] - out_matrix[[band, onset + 1]];
                p[[0, 2]] = out_matrix[[band, onset + 2]] - out_matrix[[band, onset]];
                p[[1, 2]] = out_matrix[[band, onset + 3]] - out_matrix[[band, onset + 2]];

                if nhashes > 3
                {
                    p[[0, 3]] = out_matrix[[band, onset + 1]] - out_matrix[[band, onset]];
                    p[[1, 3]] = out_matrix[[band, onset + 4]] - out_matrix[[band, onset + 1]];
                    p[[0, 4]] = out_matrix[[band, onset + 2]] - out_matrix[[band, onset]];
                    p[[1, 4]] = out_matrix[[band, onset + 4]] - out_matrix[[band, onset + 2]];
                    p[[0, 5]] = out_matrix[[band, onset + 3]] - out_matrix[[band, onset]];
                    p[[1, 5]] = out_matrix[[band, onset + 4]] - out_matrix[[band, onset + 3]];
                }
            }
            
            // For each pair emit a code
            for k in 0..6 {
                // Quantize the time deltas to 23ms
                let time_delta_1_bytes = quantized_time_for_frame_delta(p[[0, k]]);
                let time_delta_2_bytes = quantized_time_for_frame_delta(p[[1, k]]);

                // Create a key from the time deltas and the band index
                hash_material[0] = time_delta_1_bytes[0];
                hash_material[1] = time_delta_1_bytes[1];
                hash_material[2] = time_delta_2_bytes[0];
                hash_material[3] = time_delta_2_bytes[1];
                hash_material[4] = band as u8;

                let hashed_code = murmurhash2::murmurhash2(&hash_material) & HASH_BITMASK;

                // Set the code alongside the time of onset
                codes.push(CodeSegment {
                    code: hashed_code as i32,
                    time: time_for_onset_ms_quantized,
                });
            }
        }
    }

    Fingerprint::new(codes)
}

fn adaptive_onsets(onset_counter_for_band: &mut [usize], e: Matrix<f32>) -> (Matrix<i32>, usize)
{
    let mut h = [0_f32; SUBBANDS];
    let mut taus = [0_f32; SUBBANDS];
    let mut n = [0_f32; SUBBANDS];
    let mut y0 = [0_f32; SUBBANDS];
    let mut ham = [0_f32; 8];

    let mut contact = [0_usize; SUBBANDS];
    let mut lcontact = [0_usize; SUBBANDS];
    let mut tsince = [0_usize; SUBBANDS];

    let mut onset_counter = 0;

    // Take successive stretches of 8 subband samples and sum their energy under a hann window, then hop by 4 samples (50% window overlap).
    for i in 0..NSM {
        ham[i as usize] = 0.5 - 0.5 * ((2.0 * PI / (NSM - 1) as f32) * i as f32).cos();
    }

    let nc = ((e.cols as f32 / HOP as f32).floor() - ((NSM as f32 / HOP as f32).floor() - 1.0)) as usize;
    let mut eb: Matrix<f32> = Matrix::new(nc, 8);

    for i in 0..nc {
        for j in 0..SUBBANDS {
            for k in 0..NSM {
                eb[[i, j]] = eb[[i, j]] + (e[[j, (i * HOP) + k]] * ham[k]);
            }
            eb[[i, j]] = eb[[i, j]].sqrt();
        }
    }

    let frames = eb.rows;
    let bands = eb.cols;
    let pe = eb.get_array();

    let mut out_matrix: Matrix<i32> = Matrix::new(SUBBANDS, frames);

    for j in 0..bands {
        onset_counter_for_band[j] = 0;
        n[j] = 0.0;
        taus[j] = 1.0;
        h[j] = pe[j];
        contact[j] = 0;
        lcontact[j] = 0;
        tsince[j] = 0;
        y0[j] = 0.0;
    }

    let mut base_index = 0;
    for i in 0..frames {
        for j in 0..SUBBANDS {
            let mut xn = 0.0;
            // calculate the filter - FIR part
            if i >= 2 * NBN {
                for k in 0..NBN {
                    let index1 = base_index + j - SUBBANDS * k;
                    let index2 = base_index + j - SUBBANDS * (2 * NBN - k);
                    xn += BN[k] * (pe[index1] - pe[index2]);
                }
            }
            // IIR part
            xn += A1 * y0[j];
            // remember the last filtered level
            y0[j]= xn;

            contact[j] = if xn > h[j] { 1 } else { 0 };

            if contact[j] == 1 && lcontact[j] == 0 && n[j] == 0.0
            {
                // attach - record the threshold level unless we have one
                n[j] = h[j];
            }
            
            if contact[j] == 1 {
                // update with new threshold
                h[j] = xn * OVERFACT;
            }
            else
            {
                // apply decays
                h[j] *= (-1.0 / taus[j]).exp();
            }

            if contact[j] == 0 && lcontact[j] == 1
            {
                // detach
                if onset_counter_for_band[j] > 0 && out_matrix[[j, onset_counter_for_band[j] - 1]] > (i as i32 - DEADTIME as i32)
                {
                    // overwrite last-written time
                    onset_counter_for_band[j] -= 1;
                    onset_counter -= 1;
                }
                out_matrix[[j, onset_counter_for_band[j]]] = i as i32;
                onset_counter_for_band[j] += 1;
                onset_counter += 1;
                tsince[j] = 0;
            }
            tsince[j] += 1;

            if tsince[j] > TTARG
            {
                taus[j] -= 1.0;
                if taus[j] < 1.0
                {
                    taus[j] = 1.0;
                }
            }
            else
            {
                taus[j] += 1.0;
            }

            if contact[j] == 0 && tsince[j] > DEADTIME
            {
                // forget the threshold where we recently hit
                n[j] = 0.0;
            }
            lcontact[j] = contact[j];
        }
        base_index += bands;
    }

    (out_matrix, onset_counter)
}

fn quantized_time_for_frame_delta(frame: i32) -> [u8; mem::size_of::<i16>()]
{
    let time_for_frame_delta = frame as f32 / (constants::SAMPLING_RATE / 32.0);
    let left = (time_for_frame_delta * 1000.0 / QUANTIZE_DT_S).floor() * QUANTIZE_DT_S;
    let right = (QUANTIZE_DT_S * 1000.0).floor();
    let value = (left / right) as i16;
    let mut bytes = [0u8; mem::size_of::<i16>()];
    bytes.as_mut()
        .write_i16::<LittleEndian>(value)
        .expect("Unable to convert i16 to [u8]");
    bytes
}

fn quantized_time_for_frame_absolute(frame: i32, offset: i32) -> i32
{
    let time_for_frame = offset as f32 + frame as f32 / (constants::SAMPLING_RATE / 32.0);
    let left = round_to_even(time_for_frame * 1000.0 / QUANTIZE_A_S) * QUANTIZE_A_S;
    let right = (QUANTIZE_A_S * 1000.0).floor();
    (left / right) as i32
}

fn round_to_even(to_round: f32) -> f32
{
    if to_round == 0.5
    {
        // handle edge case - round to even
        if to_round.trunc() as i32 % 2 == 0
        {
            return to_round.floor();
        }
        else
        {
            return to_round.ceil();
        }
    }
    to_round.round()
}

const SUBBANDS: usize = 8;
const QUANTIZE_DT_S: f32 = 256.0 / 11025.0;
const QUANTIZE_A_S: f32 = 256.0 / 11025.0;
const HASH_BITMASK: u32 = 0x000fffff;
const TTARG: usize = 345;
const DEADTIME: usize = 128;
const BN: [f32; 3] = [0.1883, 0.4230, 0.3392];
const NBN:usize = 3;
const A1:f32 = 0.98;
const OVERFACT:f32 = 1.1;
const HOP:usize = 4;
const NSM:usize = 8;
