use block::Block;
use hashable::Hashable;

pub struct Blockchain {
    pub blocks: Vec<Block>
}

pub fn initialize() -> Blockchain {
    return Blockchain{
        blocks: Vec::new()
    }
}

pub fn add_block_with_value(chain: &mut Blockchain, value: u64) {
    let last_hash: [u8; 32];

    match chain.blocks.last() {
        Some(x) => last_hash = x.hash,
        None => {
            println!("initializing first block with empty previous hash.");
            last_hash = [0; 32];
        },
    };

    let mut new_block = Block{
        index: chain.blocks.len() as u64,
        value: value,
        timestamp: 2,
        hash: [0; 32],
        prev_hash: last_hash
    };

    new_block.hash = new_block.hash();
    chain.blocks.push( new_block );
}

pub fn is_chain_correct(chain: &Blockchain) -> bool {
    if chain.blocks.len() == 0 {
        return true;
    }
    let mut last_block = chain.blocks.first().unwrap();

    if last_block.hash != last_block.hash() {
        return false;
    }

    for block in chain.blocks.iter().skip(1) {
        let mut last_hash = last_block.hash();
        if block.prev_hash != last_hash {
            return false;
        } else {
            if block.hash() != block.hash {
                return false;
            }
        }
        last_block = block;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_correct_single_true() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);

        assert_eq!( is_chain_correct(&chain), true );
    }

    #[test]
    fn test_chain_correct_single_value_false() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
    
        {
            let block = chain.blocks.iter_mut().next().unwrap();
            block.value = 7;
        }

        assert_eq!( is_chain_correct(&chain), false );
    }

    #[test]
    fn test_chain_correct_single_timestamp_false() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
    
        {
            let block = chain.blocks.iter_mut().next().unwrap();
            block.timestamp = 7;
        }

        assert_eq!( is_chain_correct(&chain), false );
    }

    #[test]
    fn test_chain_correct_multi_true() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
        add_block_with_value(&mut chain, 7);

        assert_eq!( is_chain_correct(&chain), true );
    }

    #[test]
    fn test_chain_correct_multi_value_false() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
        add_block_with_value(&mut chain, 7);
        
        {
            let block = chain.blocks.iter_mut().last().unwrap();
            block.value = 9;
        }

        assert_eq!( is_chain_correct(&chain), false );
    }

    #[test]
    fn test_chain_correct_multi_timestamp_false() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
        add_block_with_value(&mut chain, 7);
        
        {
            let block = chain.blocks.iter_mut().last().unwrap();
            block.timestamp = 9;
        }

        assert_eq!( is_chain_correct(&chain), false );
    }

    #[test]
    fn test_chain_correct_multi_first_false() {
        let mut chain = initialize();
        add_block_with_value(&mut chain, 5);
        add_block_with_value(&mut chain, 7);
        
        {
            let block = chain.blocks.iter_mut().next().unwrap();
            block.timestamp = 9;
        }

        assert_eq!( is_chain_correct(&chain), false );
    }
}