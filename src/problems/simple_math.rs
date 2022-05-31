use std::ops::{Range, RangeInclusive};

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct SimpleMath;

impl Problem for SimpleMath {
    fn id(&self) -> u64 {
        0
    }

    fn time(&self) -> u64 {
        0 //1654056000
    }

    fn name(&self) -> &'static str {
        "Simple Math"
    }

    fn text(&self) -> &'static str {
        include_str!("text/build/simple_math")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut maths = Vec::new();

        for _ in 0..10 {
            maths.push(format!(
                "{}{}{}",
                rng.gen_range::<i32, RangeInclusive<i32>>(100..=999),
                ['+', '-', '*'][rng.gen_range::<u8, Range<u8>>(0..3) as usize],
                rng.gen_range::<i32, RangeInclusive<i32>>(100..=999)
            ));
        }

        maths.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut real = Vec::new();

        for _ in 0..10 {
            let p1 = rng.gen_range::<i32, RangeInclusive<i32>>(100..=999);
            let op = rng.gen_range::<u8, Range<u8>>(0..3);
            let p2 = rng.gen_range::<i32, RangeInclusive<i32>>(100..=999);

            real.push(
                match op {
                    0 => p1 + p2,
                    1 => p1 - p2,
                    2 => p1 * p2,
                    _ => unreachable!(),
                }
                .to_string(),
            )
        }

        real.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Problem, SimpleMath};
    use rand::RngCore;

    #[test]
    fn simple_math() {
        let seed = rand::thread_rng().next_u64();
        let math = SimpleMath.gen(seed);
        let mut out = Vec::new();

        for i in math.split(" ") {
            let p1 = &i[0..3].parse::<i32>().unwrap();
            let op = i.chars().nth(3).unwrap();
            let p2 = &i[4..7].parse::<i32>().unwrap();

            out.push(
                match op {
                    '+' => p1 + p2,
                    '-' => p1 - p2,
                    '*' => p1 * p2,
                    _ => unreachable!(),
                }
                .to_string(),
            );
        }

        assert_eq!(SimpleMath.check(seed), out.join("\n"))
    }
}
