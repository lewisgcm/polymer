extern crate blockchain;

#[cfg(test)]
mod tests {

    use blockchain::block::Block;
    use blockchain::serializable::Serializable;

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