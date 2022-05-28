use std::ops::RangeInclusive;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct NthPrime;

impl Problem for NthPrime {
    fn id(&self) -> u64 {
        9
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Nth Prime"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/nth_prime.html")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .into_iter()
            .map(|_| rng.gen_range(1..=100).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let primes_list = gen_primes(541);

        (0..10)
            .into_iter()
            .map(|_| {
                primes_list[rng.gen_range::<i32, RangeInclusive<i32>>(1..=100) as usize].to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn gen_primes(upper: usize) -> Vec<usize> {
    let mut primes_list = Vec::new();
    'm: for i in 1..=upper {
        for j in 2..i {
            if i % j < 1 {
                continue 'm;
            }
        }
        primes_list.push(i)
    }

    primes_list
}

#[cfg(test)]
mod test {
    use super::{gen_primes, NthPrime, Problem};
    use rand::RngCore;

    #[test]
    fn nth_prime() {
        let seed = rand::thread_rng().next_u64();
        let primes = NthPrime.gen(seed);
        let primes_list = gen_primes(541);

        let output = primes
            .split(" ")
            .map(|x| primes_list[x.parse::<usize>().unwrap()].to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(NthPrime.check(seed), output);
    }
}
