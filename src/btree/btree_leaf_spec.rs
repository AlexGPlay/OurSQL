use super::btree_leaf;

#[cfg(test)]
mod leaf_tests {

    #[test]
    fn test_leaf_debug(){
        let mut btree_leaf: super::btree_leaf::BTreeLeaf<u8> = super::btree_leaf::BTreeLeaf::new(5);
        assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [], children: [] }");
    }

    // Tests if the add method adds something
    #[test]
    fn test_leaf_add_1(){
        let mut btree_leaf: super::btree_leaf::BTreeLeaf<u8> = super::btree_leaf::BTreeLeaf::new(5);
        btree_leaf.add(1);
        assert_eq!(format!("{:?}", btree_leaf), "Leaf { degree: 5, keys: [1], children: [] }");
    }

}

#[cfg(test)]
mod node_tests {

    #[test]
    fn test_node_debug() {
        let mut btree_node = super::btree_leaf::BTreeNode{ key: 5 };
        assert_eq!(format!("{:?}", btree_node), "Node { key: 5 }");
    }
}