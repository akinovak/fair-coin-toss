mod alice;
mod bob;
pub mod utils;

use rand::distributions::{Distribution, Uniform};

use crate::alice::*;
use crate::bob::*;


fn main() {
    let alice = Alice::new(128);
    let mut bob = Bob::new();
    let (_x, x_2) = bob.gen_challendge(alice.n.clone());

    let (r1, r2, r3, r4) = alice.calculate_square_roots(x_2);

    let roots = vec![r1.clone(), r2.clone(), r3.clone(), r4.clone()];
    let step = Uniform::new(0, 4);
    let mut rng = rand::thread_rng();
    let choice = step.sample(&mut rng);

    let r = &roots[choice];
    println!("{}", r);

    let factors = bob.factor_n_given_square_roots(&r, &alice.n);

    if factors.is_none() {
        println!("Alice wins");
        return;
    }

    let (f1, f2) = factors.unwrap();
    assert_eq!(f1 * f2, alice.n);
    println!("Bob wins");
}