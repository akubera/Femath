///
/// A collection of algorithms
///

// fn
// add_me(i:i32, j:i32) -> i32
// {
//   i + j
// }


use std::ops::{
    Mul,
    Add
};

// use std::num;
// use std::num::Zero;
// use num::Zero;

pub fn
g_add_in_quad <T> (l:&[T]) -> T
  where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
  // where T: Copy + Mul<Output=T> + Add<&'a T, Output=T> + Zero
{
    match l.len() {
        0 => l[0],
        1 => l[0],
        _ => g_add_in_quad_recursive(l[0], &l[1..])
    }
}

pub fn
g_add_in_quad_recursive <T> (sum:T, l:&[T]) -> T
  where T: Copy + Add<T, Output=T> + Mul<T, Output=T>
{
    match l.len() {
        0 => sum,
        1 => sum + l[0] * l[0],
        _ => g_add_in_quad_recursive(sum + l[0] * l[0], &l[1..])
    }
}

pub fn
add_in_quadrature(l:&[i32]) -> i32
{
  recursive_add_in_quadrature(0, l)
}


fn
recursive_add_in_quadrature(sum:i32, list:&[i32]) -> i32
{
    let top = list[0] * list[0];

    match list.len() {
        0 => sum,
        1 => sum + top,
        _ => recursive_add_in_quadrature(sum + top, &list[1..])
    }
}
