#[cfg(test)]
mod tests {
    use oursql::btree::leaf::BTreeLeaf;

    #[test]
    fn test_leaf_debug(){
        let btree_leaf: BTreeLeaf<u8> = BTreeLeaf::new(5);
        assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [], children: [] }");
    }

    // Tests if the add method adds something
    #[test]
    fn test_leaf_add_1(){
        let mut btree_leaf: BTreeLeaf<u8> = BTreeLeaf::new(5);
        btree_leaf.add(1);
        assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [Node { key: 1 }], children: [] }");
    }

    // Tests if the add method returns 0 while the leaf has space.
    #[test]
    fn test_leaf_add_enough_space(){
        let mut btree_leaf: BTreeLeaf<u8> = BTreeLeaf::new(5);
        
        for n in 0..4 {
            let result = btree_leaf.add(n);
            assert_eq!(result, 0);
        }

        assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [Node { key: 0 }, Node { key: 1 }, Node { key: 2 }, Node { key: 3 }], children: [] }");
    }

        // Tests if the add method returns 1 for a full leaf.
        #[test]
        fn test_leaf_add_not_enough_space(){
            let mut btree_leaf: BTreeLeaf<u8> = BTreeLeaf::new(5);
            
            for n in 0..4 {
                let result = btree_leaf.add(n);
                assert_eq!(result, 0);
            }

            let result = btree_leaf.add(4);
            assert_eq!(result, 1);
    
            assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [Node { key: 0 }, Node { key: 1 }, Node { key: 2 }, Node { key: 3 }, Node { key: 4 }], children: [] }");
        }

}