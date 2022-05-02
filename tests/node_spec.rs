#[cfg(test)]
mod tests {
    use oursql::btree::node::BTreeNode;

    #[test]
    fn test_node_debug() {
        let btree_node = BTreeNode::new(5);
        assert_eq!(format!("{:?}", btree_node), "Node { key: 5 }");
    }
}