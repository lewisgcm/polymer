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

        bytes.extend_from_slice( &self.index.to_be_bytes() );
        bytes.extend_from_slice( &self.timestamp.to_be_bytes() );
        bytes.extend_from_slice( &self.value.to_be_bytes() );
        bytes.extend_from_slice( &self.prev_hash );

        return bytes;
    }
}