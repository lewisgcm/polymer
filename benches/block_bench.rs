#![feature(test)]

extern crate polymer;
extern crate test;

#[cfg(test)]
mod tests {

    use test::Bencher;
    use polymer::block::Block;
    use polymer::serializable::Serializable;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(
            || {
                let block = Block{
                    index: 0x0100000000000002,
                    timestamp: 0x0300000000000004,
                    value: 0x0500000000000006,
                    hash: [0; 32],
                    prev_hash: [0; 32]
                };
                return block.to_bytes();
            }
        );
    }
}