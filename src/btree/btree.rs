use std::fmt;
use super::leaf::BTreeLeaf;

pub struct BTree<T: Ord + fmt::Debug>{
  degree: u16,
  root: BTreeLeaf<T>,
}

impl<T: Ord + fmt::Debug> BTree<T> {

  pub fn new(degree: u16) -> Self{
    Self { degree, root: BTreeLeaf::new(degree) }
  }

  pub fn add(&mut self, key: T) -> bool{
    self.root.add(key);

    return true;
  }

}