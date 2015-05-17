///
///
/// A 3 dimensional Cartesian vector.
///

use super::PhysicalVector;
use std;

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

impl super::super::num::Zero for ThreeVector {
  fn zero() -> ThreeVector {ThreeVector {x:0.0, y:0.0, z:0.0}}
}

impl std::ops::Add for ThreeVector {
  type Output = ThreeVector;

  fn add(self, rhs:ThreeVector) -> Self::Output
  {
    ThreeVector {x: self.x + rhs.x,
                 y: self.y + rhs.y,
                 z: self.z + rhs.z}
  }
}

impl std::ops::Sub for ThreeVector {
  type Output = ThreeVector;

  fn
  sub(self, rhs:ThreeVector) -> Self::Output
  {
    ThreeVector {x: self.x - rhs.x,
                 y: self.y - rhs.y,
                 z: self.z - rhs.z}
  }
}

impl std::cmp::PartialEq for ThreeVector {

  fn eq(&self, other:&ThreeVector) -> bool
  {
    self.x == other.x && self.y == other.y && self.z == other.z
  }

}
