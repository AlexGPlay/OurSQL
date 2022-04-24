use std::fmt;
use super::btree_leaf;

pub struct BTree<T: Ord + fmt::Debug>{
  degree: u16,
  root: btree_leaf::BTreeLeaf<T>,
}

impl<T: Ord + fmt::Debug> BTree<T> {

  pub fn new(degree: u16) -> Self{
    Self { degree, root: btree_leaf::BTreeLeaf::new(degree) }
  }

  pub fn add(&mut self, key: T) -> bool{
    self.root.add(key);

    return true;
  }

}