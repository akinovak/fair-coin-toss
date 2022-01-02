use num_bigint::{BigUint};
use num_integer::Integer;
use num_primes::{Generator};

//TODO add different toss errors
#[derive(Debug)]
pub enum TossError {
    CRTError,
}


pub fn generate_congruent(security: usize) -> BigUint {
    let four = BigUint::from(4u32);
    let three = BigUint::from(3u32);
    let mut candidate = Generator::new_prime(security);
    while candidate.mod_floor(&four) != three {
        candidate = Generator::new_prime(security);
    }
    candidate
}