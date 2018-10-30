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

#[cfg(test)]
mod tests {
    use super::*;

    // Create a test struct for performing the hash
    struct TestStruct {
        data: [u8; 1]
    }

    impl Serializable for TestStruct {
        fn to_bytes(&self) -> Vec<u8> {
            return self.data.to_vec();
        }
    }

    #[test]
    fn test_hash() {
        let mut test_struct = TestStruct{
            data: [0; 1]
        };

        test_struct.data[0] = 1;
        let hash = test_struct.hash();

        assert_eq!(
            [75, 245, 18, 47, 52, 69, 84, 197, 59, 222, 46, 187, 140, 210, 183, 227, 209, 96, 10, 214, 49, 195, 133, 165, 215, 204, 226, 60, 119, 133, 69, 154],
            hash
        );
    }
}