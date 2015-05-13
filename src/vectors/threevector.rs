///
///
/// A 3 dimensional Cartesian vector.
///

use super::PhysicalVector;

pub
struct ThreeVector {
    pub x: f64
}


impl PhysicalVector for ThreeVector {

  fn mag(&self) -> f64
  {
    self.x * self.x
  }

}
