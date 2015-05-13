//
// src/vectors/mod.rs
//
// pub mod threevector::ThreeVector;
//

mod threevector;
mod lorentzvector;

pub use self::threevector::ThreeVector;
pub use self::lorentzvector::LorentzVector;

use std::num::Float;

///
/// All "physical" vectors must implement these functions.
///
pub trait PhysicalVector
{
    /// The magnitude of the vector
    /// $\norm{v}$
    fn mag(&self) -> f64
    { self.mag2().sqrt() }


    /// The magnitude of the vector, squared
    fn mag2(&self) -> f64;
}
