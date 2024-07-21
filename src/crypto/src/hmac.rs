use crate::{hash_func::HashFunc, prf::PRF};

#[derive(Clone, Copy)]
pub struct HMAC<F: HashFunc + Copy>(F);

impl<F: HashFunc + Copy> HMAC<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

impl<F: HashFunc + Copy> HMAC<F> {
    fn xor_with(key: &mut [u8], value: u8) {
        debug_assert!(F::BLOCK_SIZE <= key.len());

        for i in 0..F::BLOCK_SIZE {
            key[i] ^= value;
        }
    }

    fn xor_outer(key: &mut [u8]) {
        Self::xor_with(key, 0x5c);
    }

    fn xor_inner(key: &mut [u8]) {
        Self::xor_with(key, 0x36);
    }
}

impl<F: HashFunc + Copy> PRF for HMAC<F> {
    const OUTPUT_LEN: usize = F::OUTPUT_SIZE;

    fn apply(&self, key: &[u8], input: &[u8]) -> Vec<u8> {
        let key = if key.len() > F::BLOCK_SIZE {
            self.0.apply(key)
        } else if key.len() < F::BLOCK_SIZE {
            let mut v = key.to_vec();
            while v.len() < F::BLOCK_SIZE {
                v.push(0);
            }

            v
        } else {
            key.to_vec()
        };

        let mut outer_padding = key.clone();
        let mut inner_padding = key;

        Self::xor_inner(&mut inner_padding);
        Self::xor_outer(&mut outer_padding);

        inner_padding.extend_from_slice(input);
        let h = self.0.apply(&inner_padding);
        outer_padding.extend_from_slice(&h);

        self.0.apply(&outer_padding)
    }
}

#[cfg(test)]
mod tests {
    use sha256::Sha256;

    use crate::*;
    use super::*;

    #[test]
    fn sha256() {
        let hmac = HMAC::new(Sha256);
        let result = hmac.apply(b"key", b"The quick brown fox jumps over the lazy dog");

        assert_eq!(
            to_hex_str(&result).as_str(),
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8",
        );
    }
}
