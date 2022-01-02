use num_bigint::{BigUint, RandomBits};
use num_integer::Integer;
use rand::Rng;
use std::cmp::{max, min};

pub struct Bob {
    root: Option<BigUint>,
}

//TODO verify
impl Bob {
    pub fn new() -> Self {
        Bob { 
            root: None
        }
    }

    pub fn factor_n_given_square_roots(&self, r2: &BigUint, n: &BigUint) -> Option<(BigUint, BigUint)> {
        let r1 = self.root.as_ref().unwrap();
        if r1 == r2 || r1 == &(n - r2) {
            //alice wins
            return None
        }

        let max = max(r1, r2);
        let min = min(r1, r2);
        let f1 = (max - min).gcd(&n);
        let f2 = (max + min).gcd(&n);

        return Some((f1, f2))
    }

    pub fn gen_challendge(&mut self, n: BigUint) -> (BigUint, BigUint) {
        let mut rng = rand::thread_rng();

        let mut x: BigUint = rng.sample(RandomBits::new(n.bits()));
        while x > n {
            x = rng.sample(RandomBits::new(n.bits()))
        }
        self.root = Some(x.clone());
        (x.clone(), x.modpow(&BigUint::from(2u32), &n))
    }
}