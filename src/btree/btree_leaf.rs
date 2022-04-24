use std::fmt;

pub struct BTreeNode<T: Ord + fmt::Debug>{
  key: T,
}

pub struct BTreeLeaf<T: Ord + fmt::Debug>{
  degree: u16,
  keys: Vec<BTreeNode<T>>,
  children: Vec<BTreeLeaf<T>>,
}

impl <T: Ord + fmt::Debug> BtreeNode<T> {
  pub fn new(key: T) -> Self {
    Self { key }
  }
}

impl <T: Ord + fmt::Debug> BTreeLeaf<T> {

  pub fn new(degree: u16) -> Self {
    Self { degree, keys: vec![], children: vec![] }
  }

  /// # Add a new key to the leaf
  /// Adding a new key to the leaf means adding a new node to it.
  /// If the number of keys is the same as the degree of the leaf 1 will be returned, otherwise 0 will be returned.
  /// The idea is that the tree has to split the leaf into more leafs if 1 is returned.
  pub fn add(&mut self, key: T) -> i8 {
    self.keys.push(BTreeNode{ key });
    self.keys.sort_by(|a, b| a.key.cmp(&b.key));

    if self.keys.len() >= self.degree.into() { 
      return 1; 
    }

    return 0;
  }

}

impl<T: Ord + fmt::Debug> fmt::Debug for BTreeNode<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Node")
       .field("value", &self.key)
       .finish()
  }
}

impl<T: Ord + fmt::Debug> fmt::Debug for BTreeLeaf<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Leaf")
       .field("degree", &self.degree)
       .field("keys", &self.keys)
       .field("children", &self.children)
       .finish()
  }
}