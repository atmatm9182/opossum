use crate::{bit_vector::BitVector, hash_func::HashFunc};

/// The code for this was taken from the [wikipedia article](https://en.wikipedia.org/wiki/SHA-2#Pseudocode)
fn sha256(input: &[u8]) -> [u8; 32] {
    let mut h_arr: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let l = input.len() * 8;

    let mut vec = BitVector::new(input);
    vec.push_bit(1);

    let mut kb = 0;
    while (l + 1 + kb + 64) % 512 != 0 {
        vec.push_bit(0);
        kb += 1
    }

    vec.push_u64(l as u64);

    for chunk in vec.drain().chunks(64) {
        let mut w = [0u32; 64];

        for i in 0..16 {
            for c in 0..4 {
                w[i] <<= 8;
                w[i] |= chunk[i * 4 + c] as u32;
            }
        }

        for i in 16..64 {
            let w15 = w[i - 15];
            let s0 = w15.rotate_right(7) ^ w15.rotate_right(18) ^ (w15 >> 3);

            let w2 = w[i - 2];
            let s1 = w2.rotate_right(17) ^ w2.rotate_right(19) ^ (w2 >> 10);

            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = h_arr;

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h_arr[0] = h_arr[0].wrapping_add(a);
        h_arr[1] = h_arr[1].wrapping_add(b);
        h_arr[2] = h_arr[2].wrapping_add(c);
        h_arr[3] = h_arr[3].wrapping_add(d);
        h_arr[4] = h_arr[4].wrapping_add(e);
        h_arr[5] = h_arr[5].wrapping_add(f);
        h_arr[6] = h_arr[6].wrapping_add(g);
        h_arr[7] = h_arr[7].wrapping_add(h);
    }

    let mut result = [0; 32];
    let mut i = 0;

    for h in h_arr {
        for b in h.to_be_bytes() {
            result[i] = b;
            i += 1;
        }
    }

    result
}

#[derive(Clone, Copy)]
pub struct Sha256;

impl HashFunc for Sha256 {
    const BLOCK_SIZE: usize = 512 / 8;
    const OUTPUT_SIZE: usize = 256 / 8;

    fn apply(&self, input: &[u8]) -> Vec<u8> {
        sha256(input).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn empty_string() {
        let input = b"";
        let hash = sha256(input);

        assert_eq!(
            to_hex_str(&hash).as_str(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn quick_brown_fox() {
        let input = b"the quick brown fox jumps over the lazy dog";
        let hash = sha256(input);

        assert_eq!(
            to_hex_str(&hash).as_str(),
            "05c6e08f1d9fdafa03147fcb8f82f124c76d2f70e3d989dc8aadb5e7d7450bec"
        );
    }
}
