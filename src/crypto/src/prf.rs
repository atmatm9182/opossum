pub trait PRF: Copy {
    const OUTPUT_LEN: usize;

    fn apply(&self, key: &[u8], input: &[u8]) -> Vec<u8>;
}

