pub trait Mac {
    const OUTPUT_LEN: usize;

    fn apply(&self, key: &[u8], input: &[u8]) -> Vec<u8>;
}

