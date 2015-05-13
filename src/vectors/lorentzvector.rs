//

use super::PhysicalVector;
use super::ThreeVector;

///
/// A FourVector
///
pub
struct LorentzVector {
    /// Time component of the Lorentz-vector
    pub t: f64,

    /// Position component of the Lorentz-vector
    pub v: ThreeVector,
}


impl PhysicalVector for LorentzVector
{
  fn mag2(&self) -> f64
  {
    self.t * self.t - self.v.mag2()
  }

}
