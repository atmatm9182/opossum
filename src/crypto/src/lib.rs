mod bit_vector;
mod hash_func;
mod sha256;
mod pbkdf2;
mod hmac;
mod prf;
mod scrypt;

use std::fmt::Write;

fn to_hex_str(bytes: &[u8]) -> String {
    let mut result = String::new();

    for b in bytes {
        write!(&mut result, "{:02x}", b).unwrap();
    }

    result
}

fn xor(dest: &mut [u8], src: &[u8]) {
    debug_assert_eq!(dest.len(), src.len());

    for i in 0..dest.len() {
        dest[i] ^= src[i];
    }
}
