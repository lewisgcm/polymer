extern crate polymer;

#[cfg(test)]
mod tests {

    use polymer::blockchain::*;

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