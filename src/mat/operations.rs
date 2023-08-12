use crate::prelude::*;
use std::ops::*;

impl<const M: usize, const N: usize, T: VecEntry> Add for Matrix<M, N, T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    (self.into_vec() + rhs.into_vec()).into_matrix()
  }
}

impl<const M: usize, const N: usize, T: VecEntry> Sub for Matrix<M, N, T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    (self.into_vec() - rhs.into_vec()).into_matrix()
  }
}

impl<const M: usize, const N: usize, const P: usize, T: VecEntry> Mul<Matrix<N, P, T>> for Matrix<M, N, T> {
  type Output = Matrix<M, P, T>;

  fn mul(self, rhs: Matrix<N, P, T>) -> Self::Output {
    Vector::from_iter((0..M).map(|r| -> Vector<P, T> {
      (0..P).map(|c| {
        (self.row(r).into_vec().unwrap() *
          Vector::from_iter((rhs.col(c))
            .into_vec()
            .into_iter()
            .map(|v| v.unwrap())))
          .into_iter()
          .reduce(|acc, next| acc + next)
          .unwrap()
      }).collect()
    })).into_matrix()
  }
}

impl<const M: usize, const N: usize, T: VecEntry> Mul<Vector<N, T>> for Matrix<M, N, T> {
  type Output = Vector<M, T>;

  fn mul(self, rhs: Vector<N, T>) -> Self::Output {
    Vector::from_iter(
      (
        self * Vector::from_iter(
          rhs.into_iter().map(|v| Vector::new([v]))).into_matrix()
        )
          .into_vec()
          .into_iter()
          .map(|v| v.unwrap())
    )
  }
}
