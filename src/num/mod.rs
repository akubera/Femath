//
// src/num/mod.rs
//

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

impl Zero for i8 { fn zero() -> i8 {0} }
impl Zero for i16 { fn zero() -> i16 {0} }
impl Zero for i32 { fn zero() -> i32 {0} }
impl Zero for i64 { fn zero() -> i64 {0} }
impl Zero for isize { fn zero() -> isize {0} }

impl Zero for u8 { fn zero() -> u8 {0} }
impl Zero for u16 { fn zero() -> u16 {0} }
impl Zero for u32 { fn zero() -> u32 {0} }
impl Zero for u64 { fn zero() -> u64 {0} }
impl Zero for usize { fn zero() -> usize {0} }

impl Zero for f32 { fn zero() -> f32 {0.0} }
impl Zero for f64 { fn zero() -> f64 {0.0} }


impl One for i8 { fn one() -> i8 {1} }
impl One for i16 { fn one() -> i16 {1} }
impl One for i32 { fn one() -> i32 {1} }
impl One for i64 { fn one() -> i64 {1} }
impl One for isize { fn one() -> isize {1} }

impl One for u8 { fn one() -> u8 {1} }
impl One for u16 { fn one() -> u16 {1} }
impl One for u32 { fn one() -> u32 {1} }
impl One for u64 { fn one() -> u64 {1} }
impl One for usize { fn one() -> usize {1} }

impl One for f32 { fn one() -> f32 {1.0} }
impl One for f64 { fn one() -> f64 {1.0} }
