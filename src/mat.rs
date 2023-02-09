pub mod alias;
pub mod transform;
pub mod operations;

use crate::*;
use std::ops::{Add, Sub, Mul, Index};
pub use alias::*;
pub use transform::*;
pub use operations::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize, T: VecEntry>(Vector<M, Vector<N, T>>);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct RowIdx(usize);
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColIdx(usize);

impl<const M: usize, const N: usize, T: VecEntry> Index<(usize, usize)> for Matrix<M, N, T> {
  type Output = T;

  fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
    &self.0[row][column]
  }
}

impl<const M: usize, const N: usize, T: VecEntry> Matrix<M, N, T> {
  pub fn row(&self, index: usize) -> Matrix<1, N, T> {
    Vector::new([self.0[index]]).into_matrix()
  }

  pub fn col(&self, index: usize) -> Matrix<M, 1, T> {
    Vector::<M, Vector<1, T>>::from_iter((self.0.into_iter())
      .map(|v| Vector::new([v[index]]))
    ).into_matrix()
  }
}

impl<const M: usize, const N: usize, T: VecEntry> PartialEq for Matrix<M, N, T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
