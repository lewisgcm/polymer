use serializable::Serializable;

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub value: u64,
    pub hash: [u8; 32],
    pub prev_hash: [u8; 32]
}

impl Serializable for Block {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes : Vec<u8> = Vec::with_capacity(56);

        // We need to make sure we are mapping to the integers
        // to a consistent endianness.
        let index = u64::from_le( self.index );
        let timestamp = u64::from_le( self.timestamp );
        let value = u64::from_le( self.value );

        // Add in our index and timestamp
        for i in 0..8 {
            bytes.push( ((index >> 8*i) | 0b00000000) as u8 );
        }
        for i in 0..8 {
            bytes.push( ((timestamp >> 8*i) | 0b00000000) as u8 );
        }
        for i in 0..8 {
            bytes.push( ((value >> 8*i) | 0b00000000) as u8 );
        }

        // Add in the previous hash
        for i in 0..32 {
            bytes.push( self.prev_hash[i] );
        }
        return bytes;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_block_to_bytes() {
        let mut block = Block{
            index: 1,
            timestamp: 2,
            value: 7,
            hash: [0; 32],
            prev_hash: [0; 32]
        };
        block.prev_hash[0] = 3;
        block.prev_hash[31] = 5;
        let bytes = block.to_bytes();

        assert_eq!(
            [1, 0, 0, 0, 0, 0, 0, 0],
            bytes[0..8]
        );

        assert_eq!(
            [2, 0, 0, 0, 0, 0, 0, 0],
            bytes[8..16]
        );

        assert_eq!(
            [7, 0, 0, 0, 0, 0, 0, 0],
            bytes[16..24]
        );

        assert_eq!(
            [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5],
            bytes[24..56]
        );
    }
}