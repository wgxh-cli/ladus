#![feature(trait_alias)]

pub mod vec;
pub mod mat;

pub use vec::*;
pub use mat::*;

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn mat_mul() {
    let mat = [
      [1, 1],
      [0, 0]
    ].into_matrix();
    let vec = [114, 514].into_vec();

    assert_eq!(mat * vec, [114 + 514, 0].into_vec());
  }
}
