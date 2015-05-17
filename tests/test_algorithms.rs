/*
 *  tests/test_algorithms.rs
 */

extern crate femath;

use femath::algorithms::{
    add_in_quadrature
};

#[test]
fn
test_add()
{
    let data = [1, 2, 3];

    let z:i32 = add_in_quadrature(&data);
    assert_eq!(z, 14);

    let y = add_in_quadrature(&[1., 1., 1.]);
    assert_eq!(y, 3.);

    assert_eq!(add_in_quadrature(&[3., 1., 2.]), 14.);

    assert_eq!(add_in_quadrature(&[0.]), 0.);

    assert_eq!(add_in_quadrature(&[1]), 1);

    assert_eq!(add_in_quadrature(&[2]), 4);

    assert_eq!(add_in_quadrature(&[2, 2]), 8);
}
