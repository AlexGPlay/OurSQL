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

}