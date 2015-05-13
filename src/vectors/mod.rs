//
// src/vectors/mod.rs
//
// pub mod threevector::ThreeVector;
//

mod threevector;

pub use self::threevector::ThreeVector;

/// All "physical" vectors must implement these functions.
pub trait PhysicalVector {

    /// The magnitude of the vector
    /// $\norm{v}$
    fn mag(&self) -> f64;
}
