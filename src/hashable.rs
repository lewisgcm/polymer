use openssl::sha;
use serializable::Serializable;

pub trait Hashable {
    fn hash(&self) -> [u8; 32];
}

impl<T> Hashable for T where T: Serializable {
    fn hash(&self) -> [u8; 32] {
        let mut sha2 = sha::Sha256::new();
        sha2.update( &self.to_bytes() );
        return sha2.finish();
    }
}