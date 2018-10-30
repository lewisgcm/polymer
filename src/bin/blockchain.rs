//extern crate openssl;

extern crate blockchain;

/*mod blockchain;
mod serializable;
mod block;
mod hashable;*/

fn main() {
    let mut _chain = blockchain::blockchain::initialize();
    blockchain::blockchain::add_block_with_value(&mut _chain, 5);
    blockchain::blockchain::add_block_with_value(&mut _chain, 3);
    blockchain::blockchain::add_block_with_value(&mut _chain, 8);
    blockchain::blockchain::is_chain_correct(&mut _chain);
}
