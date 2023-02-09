pub mod alias;
pub mod transform;
pub mod operations;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub, Mul, Div, Index};
use std::collections::VecDeque;
pub use transform::*;
pub use alias::*;
pub use operations::*;

// First let's start with some basic types.

/// The `VecEntry` implements the traits that a vector's entry should implement.
/// It's just a shortcut there.
pub trait VecEntry = Sized + Copy + Clone + Debug
  + PartialEq
  + Add<Self, Output = Self>
  + Sub<Self, Output = Self>
  + Mul<Self, Output = Self>
  + Div<Self, Output = Self>;

#[derive(Debug, Clone, Copy, Hash)]
pub struct Vector<const L: usize, T> {
  els: [T; L],
}
#[derive(Debug, Clone)]
pub struct VecIter<T: VecEntry>(VecDeque<T>);



// And then, let's implements them some traits.
impl<const L: usize, T> Vector<L, T> {
  pub fn new(els: [T; L]) -> Self {
    Vector {
      els,
    }
  }
}

impl<const L: usize, T: VecEntry> Vector<L, T> {
  pub fn with_nth(self, index: usize, value: T) -> Self {
    Vector::from_iter(self.into_iter()
      .enumerate()
      .map(|(idx, v)| if idx == index {
        value
      } else {
        v
      }))
  }
}

impl<T: VecEntry> Vector<1, T> {
  pub fn unwrap(self) -> T {
    self.els[0]
  }

}

impl<const L: usize, T: VecEntry> Index<usize> for Vector<L, T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.els[index]
  }
}

impl<const L: usize, T: VecEntry> PartialEq for Vector<L, T> {
  fn eq(&self, other: &Self) -> bool {
    self.els == other.els
  }
}

impl<const L: usize> Vector<L, f32> {
  pub fn round(self) -> Vector<L, f32> {
    self.els.into_iter()
      .map(|f| f.round())
      .collect()
  }
}

