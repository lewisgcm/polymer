extern crate polymer;

fn main() {
    let mut _chain = polymer::blockchain::initialize();
    polymer::blockchain::add_block_with_value(&mut _chain, 5);
    polymer::blockchain::add_block_with_value(&mut _chain, 3);
    polymer::blockchain::add_block_with_value(&mut _chain, 8);
    polymer::blockchain::is_chain_correct(&mut _chain);
}
