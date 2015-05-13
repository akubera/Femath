
extern crate femath;

use femath::vectors::{
    ThreeVector
};

#[test]
fn test_threevector()
{
    let v = ThreeVector {x: 1.0};
    assert!(v.x == 1.0);
}
