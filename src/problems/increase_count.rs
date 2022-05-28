use std::ops::Range;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct IncreaseCount;

impl Problem for IncreaseCount {
    fn id(&self) -> u64 {
        1
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Increase Count"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/increase_count.html")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut sub = Vec::new();

            for _ in 0..rng.gen_range(10..20) {
                sub.push(rng.gen_range(0..10000).to_string());
            }
            out.push(sub.join(","));
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut sub = Vec::new();

            for _ in 0..rng.gen_range::<i32, Range<i32>>(10..20) {
                sub.push(rng.gen_range::<i32, Range<i32>>(0..10000));
            }
            out.push(
                sub.windows(2)
                    .fold(0, |inc, x| {
                        if x[1] > x[0] {
                            return inc + 1;
                        }
                        inc
                    })
                    .to_string(),
            );
        }

        out.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{IncreaseCount, Problem};
    use rand::RngCore;

    #[test]
    fn nth_prime() {
        let seed = rand::thread_rng().next_u64();
        let nums = IncreaseCount.gen(seed);
        let mut out = Vec::new();

        println!("{}", nums);
        for i in nums.split(" ") {
            let mut inc = 0;
            for j in i
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .windows(2)
            {
                if j[1] > j[0] {
                    inc += 1;
                }
            }

            out.push(inc.to_string());
        }

        assert_eq!(out.join("\n"), IncreaseCount.check(seed));
    }
}
