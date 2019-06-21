const P: usize = 40;
const ARRAY_SIZE: usize = P + 1;
const ALPHA: f32 = 1.0 / 8.0;

pub fn compute(samples: &[f32]) -> Vec<f32>
{
    let mut r = [0.0; ARRAY_SIZE];
    r[0] = 0.001;
    let mut xo = [0.0; ARRAY_SIZE];
    let mut ai = [0.0; ARRAY_SIZE];

    let mut whitened = vec![0.0; samples.len()];

    let numsamples = samples.len();
    let blocklen = 10000;
    let mut newblocklen: usize;

    for i in (0..numsamples).step_by(blocklen) {
        if i + blocklen >= numsamples {
            newblocklen = numsamples - i - 1;
        } else {
            newblocklen = blocklen;
        }
        compute_block(i, newblocklen, samples, &mut r, &mut xo, &mut ai, &mut whitened);
    }

    whitened
}

fn compute_block(start: usize, block_size: usize, samples: &[f32], r: &mut [f32], xo: &mut [f32], ai: &mut [f32], whitened: &mut Vec<f32>)
{
    // calculate autocorrelation of current block
    for i in 0..=P {
        let mut acc = 0.0;
        for j in i..block_size {
            acc = acc + samples[j + start] * samples[j - i + start];
        }
        // smoothed update
        r[i] = r[i] + ALPHA * (acc - r[i]);
    }

    // calculate new filter coefficients
    // Durbin's recursion, per p. 411 of Rabiner & Schafer 1978
    let mut e = r[0];
    let mut ki: f32;
    for i in 1..=P {
        let mut sum_alpha_r = 0.0;
        for j in 1..i {
            sum_alpha_r = sum_alpha_r + ai[j] * r[i - j];
        }
        ki = (r[i] - sum_alpha_r) / e;
        ai[i] = ki;
        for j in 1..=(i / 2) {
            let aj = ai[j];
            let aimj = ai[i - j];
            ai[j] = aj - ki * aimj;
            ai[i - j] = aimj - ki * aj;
        }
        e = (1.0 - ki.powi(2)) * e;
    }

    // calculate new output
    for i in 0..block_size {
        let mut acc = samples[i + start];
        let mut minip = i;
        if P < minip {
            minip = P;
        }

        for j in (i+1)..=P {
            acc = acc - ai[j] * xo[P + i - j];
        }
        for j in 1..=minip {
            acc = acc - ai[j] * samples[i - j + start];
        }
        whitened[i + start] = acc;
    }

    // save last few frames of input
    for i in 0..=P {
        xo[i] = samples[block_size - 1 - P + i + start];
    }
}