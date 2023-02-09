use super::*;

pub type Vec2<T> = Vector<2, T>;
pub type Vec3<T> = Vector<3, T>;
pub type Vec4<T> = Vector<4, T>;

impl<T: VecEntry> Vec2<T> {
  #[inline]
  pub fn x(&self) -> T {
    self[0]
  }

  #[inline]
  pub fn y(&self) -> T {
    self[1]
  }
}

impl<T: VecEntry> Vec3<T> {
  #[inline]
  pub fn x(&self) -> T {
    self[0]
  }

  #[inline]
  pub fn y(&self) -> T {
    self[1]
  }

  #[inline]
  pub fn z(&self) -> T {
    self[2]
  }
}

impl<T: VecEntry> IntoVector<2, T> for (T, T) {
  fn into_vec(self) -> Vec2<T> {
    [self.0, self.1].into_vec()
  }
}

impl<T: VecEntry> IntoVector<3, T> for (T, T, T) {
  fn into_vec(self) -> Vec3<T> {
    [self.0, self.1, self.2].into_vec()
  }
}

impl<T: VecEntry> IntoVector<4, T> for (T, T, T, T) {
  fn into_vec(self) -> Vec4<T> {
    [self.0, self.1, self.2, self.3].into_vec()
  }
}

impl<T: VecEntry> IntoVector<3, T> for Vec4<T> {
  fn into_vec(self) -> Vec3<T> {
    (self[0], self[1], self[2]).into_vec()
  }
}

impl<T: VecEntry> IntoVector<2, T> for Vec3<T> {
  fn into_vec(self) -> Vec2<T> {
    (self[0], self[1]).into_vec()
  }
}

impl<T: VecEntry + Default> IntoVector<3, T> for Vec2<T> {
  fn into_vec(self) -> Vec3<T> {
    (self[0], self[1], T::default()).into_vec()
  }
}

impl<T: VecEntry + Default> IntoVector<4, T> for Vec3<T> {
  fn into_vec(self) -> Vec4<T> {
    (self[0], self[1], self[2], T::default()).into_vec()
  }
}
