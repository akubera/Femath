///
/// A collection of algorithms
///

use super::num::{
  Zero,
  One
};

use std::ops::{
    Mul,
    Add
};

pub mod primes;

pub fn
add_in_quadrature <T> (l:&[T]) -> T
  where T: Copy + Add<T, Output=T> + Mul<T, Output=T> + Zero
  // where T: Copy + Mul<Output=T> + Add<&'a T, Output=T> + Zero
{
    match l.len() {
        0 => T::zero(),
        1 => l[0] * l[0],
        _ => recursive_add_in_quadrature(l[0] * l[0], &l[1..])
    }
}

fn
recursive_add_in_quadrature <T> (sum:T, l:&[T]) -> T
  where T: Copy + Add<T, Output=T> + Mul<T, Output=T> + Zero
{
    match l.len() {
        0 => sum,
        1 => sum + l[0] * l[0],
        _ => recursive_add_in_quadrature(sum + l[0] * l[0], &l[1..])
    }
}

pub fn
geometric_mean <T> (l:&[T]) -> T
  where T: Copy + Mul<T, Output=T> + One + Pow
{
  let product = recursive_product(T::one(), &l);
  product.pow(3)
}

fn
recursive_product <T> (product:T, l:&[T]) -> T
  where T: Copy + Mul<T, Output=T>
{
  match l.len() {
    0 => product,
    1 => product * l[0],
    _ => recursive_product(product * l[0], &l[1..])
  }
}
