// source taken from: https://docs.rs/crate/murmurhash32/0.2.0/source/src/murmurhash2.rs
use byteorder::{ByteOrder, LittleEndian};
const M: u32 = 0x5bd1_e995;
const SEED: u32 = 0x9ea5fa36;

pub fn murmurhash2(mut key: &[u8]) -> u32 {
    let len = key.len() as u32;
    let mut h: u32 = SEED ^ len;

    let num_blocks = len / 4;
    for _ in 0..num_blocks {
        let mut k: u32 = LittleEndian::read_u32(key);
        k = k.wrapping_mul(M);
        k ^= k >> 24;
        k = k.wrapping_mul(M);
        h = h.wrapping_mul(M);
        h ^= k;
        key = &key[4..];
    }

    // Handle the last few bytes of the input array
    // let remaining: &[u8] = &key[key.len() & !3..];
    match key.len() {
        3 => {
            h ^= u32::from(key[2]) << 16;
            h ^= u32::from(key[1]) << 8;
            h ^= u32::from(key[0]);
            h = h.wrapping_mul(M);
        }
        2 => {
            h ^= u32::from(key[1]) << 8;
            h ^= u32::from(key[0]);
            h = h.wrapping_mul(M);
        }
        1 => {
            h ^= u32::from(key[0]);
            h = h.wrapping_mul(M);
        }
        _ => {}
    }
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^ (h >> 15)
}