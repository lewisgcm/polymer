extern crate polymer;

#[cfg(test)]
mod tests {

    use polymer::block::Block;
    use polymer::serializable::Serializable;

    #[test]
    fn test_block_to_bytes() {
        let mut block = Block{
            index: 0x0100000000000002,
            timestamp: 0x0300000000000004,
            value: 0x0500000000000006,
            hash: [0; 32],
            prev_hash: [0; 32]
        };
        block.prev_hash[0] = 7;
        block.prev_hash[31] = 8;
        let bytes = block.to_bytes();

        assert_eq!(
            [1, 0, 0, 0, 0, 0, 0, 2],
            bytes[0..8]
        );

        assert_eq!(
            [3, 0, 0, 0, 0, 0, 0, 4],
            bytes[8..16]
        );

        assert_eq!(
            [5, 0, 0, 0, 0, 0, 0, 6],
            bytes[16..24]
        );

        assert_eq!(
            [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8],
            bytes[24..56]
        );
    }
}