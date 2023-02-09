use super::*;

impl<const L: usize, T: VecEntry> Add for Vector<L, T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    self.into_iter()
      .zip(rhs.into_iter())
      .map(|(x1, x2)| x1 + x2)
      .collect()
  }
}

impl<const L: usize, T: VecEntry> Sub for Vector<L, T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    self.into_iter()
      .zip(rhs.into_iter())
      .map(|(x1, x2)| x1 - x2)
      .collect()
  }
}

impl<const L: usize, T: VecEntry> Mul for Vector<L, T> {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    self.into_iter()
      .zip(rhs.into_iter())
      .map(|(x1, x2)| x1 * x2)
      .collect()
  }
}

impl<const L: usize, T: VecEntry> Div for Vector<L, T> {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    self.into_iter()
      .zip(rhs.into_iter())
      .map(|(x1, x2)| x1 / x2)
      .collect()
  }
}

impl<const L: usize, T: VecEntry> Mul<T> for Vector<L, T> {
  type Output = Self;
  
  fn mul(self, rhs: T) -> Self::Output {
    self.into_iter()
      .map(|x| x * rhs)
      .collect()
  }
}

impl<const L: usize, T: VecEntry> Div<T> for Vector<L, T> {
  type Output = Self;
  
  fn div(self, rhs: T) -> Self::Output {
    self.into_iter()
      .map(|x| x / rhs)
      .collect()
  }
}

