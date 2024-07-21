#[derive(Debug)]
pub struct BitVector {
    buf: Vec<u8>,
    bit_len: usize,
}

impl BitVector {
    pub fn new(init: &[u8]) -> Self {
        Self {
            buf: Vec::from(init),
            bit_len: init.len() * 8,
        }
    }

    pub fn push_bit(&mut self, bit: u8) {
        self.bit_len += 1;

        let desired_len = self.byte_len();
        let actual_len = self.buf.len();

        if desired_len > actual_len {
            self.buf.push(0);
        }

        if bit == 0 {
            return;
        }

        let bit_idx = (self.bit_len - 1) % 8;
        let buf_len = self.buf.len();

        self.buf[buf_len - 1] |= 0x80 >> bit_idx;
    }

    pub fn drain(self) -> Vec<u8> {
        self.buf
    }

    pub fn push_byte(&mut self, mut b: u8) {
        for _ in 0..8 {
            self.push_bit(b & 0x80);
            b <<= 1;
        }
    }

    pub fn push_u64(&mut self, v: u64) {
        for b in v.to_be_bytes() {
            self.push_byte(b);
        }
    }

    fn byte_len(&self) -> usize {
        (self.bit_len as f64 / 8.0).ceil() as usize
    }
}
#[cfg(test)]
mod tests {
    use crate::bit_vector::*;

    #[test]
    fn push_bit() {
        let mut vec = BitVector::new(&[]);

        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(0);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(0);
        vec.push_bit(1);
        vec.push_bit(0);

        assert_eq!(vec.drain(), vec![0b11011010]);

        let mut vec = BitVector::new(&[0xFF]);

        vec.push_bit(0);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(1);
        vec.push_bit(1);

        assert_eq!(vec.drain(), vec![0xFF, 0x7F]);
    }

    #[test]
    fn push_byte() {
        let mut vec = BitVector::new(&[]);

        vec.push_byte(1);
        vec.push_byte(0);
        vec.push_byte(0x6C);

        assert_eq!(vec.drain(), vec![1, 0, 0x6C]);
    }

    #[test]
    fn push_u64() {
        let mut vec = BitVector::new(&[]);

        vec.push_u64(0);

        assert_eq!(vec.drain(), vec![0, 0, 0, 0, 0, 0, 0, 0]);

        let mut vec = BitVector::new(&[]);

        vec.push_u64(0xDEADBEEF_FAC198AB);

        assert_eq!(
            vec.drain(),
            vec![0xDE, 0xAD, 0xBE, 0xEF, 0xFA, 0xC1, 0x98, 0xAB]
        );
    }
}
