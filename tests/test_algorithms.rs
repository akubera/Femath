/*
 *  tests/test_algorithms.rs
 */

extern crate femath;

use femath::algorithms::{
    add_in_quadrature,
    g_add_in_quad
};

#[test]
fn
test_add()
{
    let data = [1, 2, 3];

    let z:i32 = g_add_in_quad(&data);
    assert_eq!(z, 14);

    let y = g_add_in_quad(&[1., 1., 1.]);
    assert_eq!(y, 3.);
}
