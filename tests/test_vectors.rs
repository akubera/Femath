
extern crate femath;

use femath::vectors::{
  ThreeVector,
  LorentzVector,
  PhysicalVector
};

#[test]
fn test_threevector()
{
  let v = ThreeVector {x: 1.0, y: 2.0, z: 3.0};
  assert_eq!(v.x, 1.0);
  assert_eq!(v.mag2(), 14.0);
  assert_eq!(v.mag(), (14.0 as f64).sqrt());
}


#[test]
fn test_lorentz_vector_constructor()
{
  let v = LorentzVector {t:3., v: ThreeVector{x: 1.0, y: 1.0, z: 1.0}};
  assert_eq!(v.t, 3.0);
  assert_eq!(v.mag2(), 6.0);
}
