use std::fmt;

pub struct BTreeNode<T: Ord + fmt::Debug>{
  pub key: T,
}

impl <T: Ord + fmt::Debug> BTreeNode<T> {
  pub fn new(key: T) -> Self {
    Self { key }
  }
}

impl<T: Ord + fmt::Debug> fmt::Debug for BTreeNode<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Node")
       .field("key", &self.key)
       .finish()
  }
}