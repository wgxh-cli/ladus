use crate::prelude::*;

pub trait FromMatrix<const M: usize, const N: usize, T: VecEntry> {
  fn from_matrix(mat: Matrix<M, N, T>) -> Self;
}

pub trait IntoMatrix<const M: usize, const N: usize, T: VecEntry> {
  fn into_matrix(self) -> Matrix<M, N, T>;
}



impl<const M: usize, const N: usize, T: VecEntry> IntoMatrix<M, N, T> for [[T; N]; M] {
  fn into_matrix(self) -> Matrix<M, N, T> {
    Matrix(
      self.into_iter()
        .map(|a| a.into_vec())
        .collect()
    )
  }
}

impl<const M: usize, const N: usize, T: VecEntry> FromMatrix<M, N, T> for Vector<M, Vector<N, T>> {
  fn from_matrix(mat: Matrix<M, N, T>) -> Self {
    mat.0
  }
}

impl<const M: usize, const N: usize, T: VecEntry> IntoVector<M, Vector<N, T>> for Matrix<M, N, T> {
  fn into_vec(self) -> Vector<M, Vector<N, T>> {
    self.0
  }
}

impl<const M: usize, const N: usize, T: VecEntry> IntoMatrix<M, N, T> for Vector<M, Vector<N, T>> {
  fn into_matrix(self) -> Matrix<M, N, T> {
    Matrix(self)
  }
}
