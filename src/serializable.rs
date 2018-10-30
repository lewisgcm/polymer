pub trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
}