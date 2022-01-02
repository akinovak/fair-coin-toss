use num_bigint::{BigUint};
use num_integer::Integer;
use num_primes::{Generator, Verification};

#[derive(Debug)]
pub enum TossError {
    CRTError,
    PrimeError,
    WrongFactors
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

pub fn verify_prime(p: BigUint) -> Result<(), TossError> {
    if !Verification::is_prime(&p) {
        println!("not prime");
        return Err(TossError::PrimeError)
    }

    if p.mod_floor(&BigUint::from(4u32)) != BigUint::from(3u32) {
        println!("not congruent");
        return Err(TossError::PrimeError)
    }

    Ok(())
}