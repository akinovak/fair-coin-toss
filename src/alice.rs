use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use crate::utils::{generate_congruent, TossError};

pub struct Alice {
    p: BigUint,
    q: BigUint,
    pub n: BigUint,
}

impl Alice {
    pub fn new(security: usize) -> Self {
        let p = generate_congruent(security);
        let q = generate_congruent(security);
        let n = &p * &q;
        Alice {
            p, 
            q, 
            n,
        }
    }

    pub fn calculate_square_roots(&self, challendge: BigUint) -> (BigUint, BigUint, BigUint, BigUint) {
        let one = BigUint::from(1u32);
        let four = BigUint::from(4u32);
        let p_degree = (&self.p + &one).div_floor(&four);
        let q_degree = (&self.q + &one).div_floor(&four);

        let x1 = challendge.modpow(&p_degree, &self.p);
        let x2 = &self.p - &x1;
        let x3 = challendge.modpow(&q_degree, &self.q);
        let x4 = &self.q - &x3;

        //CRT
        let r1 = self.crt_for_tow(x1.clone(), x3.clone()).unwrap();
        let r2 = self.crt_for_tow(x1.clone(), x4.clone()).unwrap();
        let r3 = self.crt_for_tow(x2.clone(), x3.clone()).unwrap();
        let r4 = self.crt_for_tow(x2.clone(), x4.clone()).unwrap();

        (r1, r2, r3, r4)
    }

    fn crt_for_tow(
        &self,
        congruence_1: BigUint,
        congruence_2: BigUint,
    ) -> Result<BigUint, TossError> {
        let p = BigInt::from_biguint(Sign::Plus, self.p.clone());
        let q = BigInt::from_biguint(Sign::Plus, self.q.clone());
        let c1 = BigInt::from_biguint(Sign::Plus, congruence_1);
        let c2 = BigInt::from_biguint(Sign::Plus, congruence_2);
        let n = BigInt::from_biguint(Sign::Plus, self.n.clone());

        let e = p.extended_gcd(&q);
        let x = (e.x * &p).mod_floor(&n);
        let y = (e.y * &q).mod_floor(&n);

        match (c1 * y + c2 * x).mod_floor(&n).to_biguint() {
            Some(x) => Ok(x),
            None => Err(TossError::CRTError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Alice;
    use num_bigint::{BigUint, RandomBits};
    use rand::Rng;
    #[test]
    fn generate_new() {
        let security = 128;

        let _sc = Alice::new(security);
    }

    #[test]
    fn roots() {
        let security = 128;
        let sc = Alice::new(security);

        let mut rng = rand::thread_rng();

        let mut x: BigUint = rng.sample(RandomBits::new(sc.n.bits()));
        while x > sc.n {
            x = rng.sample(RandomBits::new(sc.n.bits()))
        }
        let challendge = x.modpow(&BigUint::from(2u32), &sc.n);

        let (_r1, _r2, _r3, _r4) = sc.calculate_square_roots(challendge);
    }
}
