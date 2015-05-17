
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
fn test_threevector_equality()
{
  let a = ThreeVector {x: 1.0, y: 2.0, z: 3.0};
  let b = ThreeVector {x: 1.0, y: 2.0, z: 3.0};
  let r = a == b;
  assert!(r);
}

#[test]
fn test_threevector_addition()
{
  let u = ThreeVector {x: 1.0, y: 2.0, z: 3.0};
  let v = ThreeVector {x: 1.0, y: 2.0, z: 3.0};
  let s = ThreeVector {x: 2.0, y: 4.0, z: 6.0};
  let r = u + v;
  let q = r == s;
  assert!(q);
}



#[test]
fn test_lorentz_vector_constructor()
{
  let v = LorentzVector {t:3., v: ThreeVector{x: 1.0, y: 1.0, z: 1.0}};
  assert_eq!(v.t, 3.0);
  assert_eq!(v.mag2(), 6.0);
}
