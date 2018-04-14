pub trait ImageHeader {
    fn get_bytes(&self) -> Vec<u8>;
}