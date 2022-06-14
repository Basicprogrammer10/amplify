use std::ops::RangeInclusive;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct ConwaysGameOfLife;

impl Problem for ConwaysGameOfLife {
    fn id(&self) -> u64 {
        14
    }

    fn time(&self) -> u64 {
        0
    }

    fn name(&self) -> &'static str {
        "Conway's Game of Life"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        todo!()
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        todo!()
    }
}

// 7 [
//     [----------],
//     [----------],
//     [----------],
//     [----------],
//     [----------],
//     [----------],
//     [---X------],
//     [---X------],
//     [---X------],
//     [----------]
// ]

// #[cfg(test)]
// mod test {
//     use super::{gen_primes, NthPrime, Problem};
//     use rand::RngCore;

//     #[test]
//     fn nth_prime() {
//         let seed = rand::thread_rng().next_u64();
//     }
// }
