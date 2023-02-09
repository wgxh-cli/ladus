use super::*;

pub trait FromVector<const L: usize, T> {
  fn from_vec(vector: Vector<L, T>) -> Self;
}

pub trait IntoVector<const L: usize, T> {
  fn into_vec(self) -> Vector<L, T>;
}

impl<const L: usize, T: VecEntry> FromIterator<T> for Vector<L, T> {
  fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
    Vector::new(
      *<Box<[T; L]>>::try_from(
        iter
          .into_iter()
          .collect::<Box<[T]>>()
      ).unwrap()
    )
  }
}

impl<T: VecEntry> Iterator for VecIter<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop_front()
  }
}

impl<const L: usize, T: VecEntry> IntoIterator for Vector<L, T> {
  type Item = T;
  type IntoIter = VecIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    VecIter(self.els.into_iter().collect())
  }
}

impl<const L: usize, T: VecEntry> IntoVector<L, T> for [T; L] {
  fn into_vec(self) -> Vector<L, T> {
    self.into_iter().collect()
  }
}

