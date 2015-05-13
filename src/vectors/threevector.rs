///
///
/// A 3 dimensional Cartesian vector.
///

use super::PhysicalVector;

pub
struct ThreeVector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl PhysicalVector for ThreeVector {

  fn mag2(&self) -> f64
  {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

}
