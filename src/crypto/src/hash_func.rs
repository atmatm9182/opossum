pub trait HashFunc {
    const BLOCK_SIZE: usize;
    const OUTPUT_SIZE: usize;

    fn apply(&self, input: &[u8]) -> Vec<u8>;
}

