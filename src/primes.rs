
fn
recursive_factor(n : u64, x : u64) -> u64
{
  if n % x == 0 { x }
  else { recursive_factor(n, x+1) }
}

pub fn
firstfac(n : u64) -> u64
{
  if n % 2 == 0 { 2 }
  else { recursive_factor(n, 3) }
}

pub fn
is_prime(n : u64) -> bool
{
  if n <= 1 {false}
  else {firstfac(n) == n}
}
