mod bit_vector;
mod hash_func;
mod sha256;
mod pbkdf2;
mod hmac;
mod prf;

use std::fmt::Write;

use prf::PRF;

fn to_hex_str(bytes: &[u8]) -> String {
    let mut result = String::new();

    for b in bytes {
        write!(&mut result, "{:02x}", b).unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
}
