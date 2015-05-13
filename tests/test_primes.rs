
extern crate femath;

use femath::primes::{
    is_prime
};

#[test]
fn test_primes()
{
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(29));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(100));
    assert!(!is_prime(27));
}
