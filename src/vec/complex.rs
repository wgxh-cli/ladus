use crate::prelude::*;
use std::ops::*;

#[derive(Debug)]
pub struct Complex<T>(Vec2<T>);
impl<T: VecEntry> Complex<T> {
  pub fn real(&self) -> T {
    self.0.x()
  }
  pub fn imag(&self) -> T {
    self.0.y()
  }
}
impl Complex<f32> {
  pub fn abs(&self) -> f32 {
    (self.real().powi(2) + self.imag().powi(2)).sqrt()
  }
}

impl<T> FromVector<2, T> for Complex<T> {
  fn from_vec(vector: Vector<2, T>) -> Self {
    Complex(vector)
  }
}

impl<T> IntoVector<2, T> for Complex<T> {
  fn into_vec(self) -> Vector<2, T> {
    self.0
  }
}

impl<T: VecEntry> Add<Self> for Complex<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    let (a, b) = (self.real(), self.imag());
    let (c, d) = (rhs.real(), rhs.imag());
    Complex::from_vec(Vec2::new([
      a + c,
      b + d,
    ]))
  }
}

impl<T: VecEntry> Sub<Self> for Complex<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    let (a, b) = (self.real(), self.imag());
    let (c, d) = (rhs.real(), rhs.imag());
    Complex::from_vec(Vec2::new([
      a - c,
      b - d,
    ]))
  }
}

impl<T: VecEntry> Mul<Self> for Complex<T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    let (a, b) = (self.real(), self.imag());
    let (c, d) = (rhs.real(), rhs.imag());
    Complex::from_vec(Vec2::new([
      a * c - b * d,
      a * d - b * c
    ]))
  }
}
