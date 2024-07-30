use crate::{prf::Mac, xor};

pub fn pbkdf2<M: Mac>(prf: &M, password: &[u8], salt: &[u8], c: usize, dk_len: usize) -> Vec<u8> {
    let nchunks = (dk_len as f32 / (M::OUTPUT_LEN as f32 * 8.0)).ceil() as usize;

    fn one_block<F: Mac>(prf: &F, password: &[u8], salt: &[u8], c: usize, i: u32) -> Vec<u8> {
        let mut salt = prf.apply(password, &[salt, &i.to_be_bytes()].concat());
        let mut accum = salt.clone();

        for _ in 1..c {
            salt = prf.apply(password, &salt);
            xor(&mut accum, &salt);
        }

        accum
    }

    let mut res = vec![];
    for i in 0..nchunks {
        let one = one_block(prf, password, salt, c, i as u32 + 1);
        res.extend_from_slice(&one);
    }

    if dk_len < res.len() {
        res[0..dk_len].to_vec()
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use hmac::Hmac;
    use sha256::Sha256;

    use super::*;
    use crate::*;

    #[test]
    fn hmac_sha256() {
        let password = b"p4$$w0rD";
        let salt = b"abcdefgh";

        let hmac = Hmac::new(Sha256);

        let result = pbkdf2(&hmac, password, salt, 1000, 256);
        assert_eq!(
            to_hex_str(&result).as_str(),
            "ac612a91ab621b800f38df5e87d093da8615fa6bacdad6532a9d31b70e2bc242",
        );
    }

    #[test]
    fn hmac_sha256_dk_len_128() {
        let password = b"pass";
        let salt = b"salt1234";

        let hmac = Hmac::new(Sha256);

        let result = pbkdf2(&hmac, password, salt, 1, 128);
        assert_eq!(
            to_hex_str(&result).as_str(),
            "b952112bb3ae1794ede803b60800daf2766c73bec626e1f24c55d751ff41079a"
        );
    }
}
