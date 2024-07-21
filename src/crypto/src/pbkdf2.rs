use crate::prf::PRF;

fn xor(dest: &mut [u8], src: &[u8]) {
    debug_assert_eq!(dest.len(), src.len());

    for i in 0..dest.len() {
        dest[i] ^= src[i];
    }
}

fn pbkdf2<F: PRF>(prf: &F, password: &[u8], salt: &[u8], c: usize, dk_len: usize) -> Vec<u8> {
    let nchunks = dk_len / (F::OUTPUT_LEN * 8);

    fn one_block<F: PRF>(prf: &F, password: &[u8], salt: &[u8], c: usize, i: u32) -> Vec<u8> {
        let mut salt = prf.apply(password, &[salt, &i.to_be_bytes()].concat());
        let mut accum = salt.clone();

        for _ in 1..c {
            salt = prf.apply(password, &salt);
            xor(&mut accum, &salt);
        }

        accum
    }

    let mut res = vec![];
    for i in 1..=nchunks {
        let one = one_block(prf, password, salt, c, i as u32);
        res.extend_from_slice(&one);
    }

    return res;
}

#[cfg(test)]
mod tests {
    use hmac::HMAC;
    use sha256::Sha256;

    use super::*;
    use crate::*;

    #[test]
    fn hmac_sha256() {
        let password = b"p4$$w0rD";
        let salt = b"abcdefgh";

        let hmac = HMAC::new(Sha256);

        let result = pbkdf2(&hmac, password, salt, 1000, 256);
        assert_eq!(
            to_hex_str(&result).as_str(),
            "ac612a91ab621b800f38df5e87d093da8615fa6bacdad6532a9d31b70e2bc242",
        );
    }
}
