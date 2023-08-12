#![feature(trait_alias)]

pub mod vec;
pub mod mat;
pub mod prelude;

#[cfg(test)]
mod tests {
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
