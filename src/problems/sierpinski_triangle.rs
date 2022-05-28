use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct SierpinskiTriangle;

impl Problem for SierpinskiTriangle {
    fn id(&self) -> u64 {
        4
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Sierpiński Triangle"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/sierpinski_triangle.html")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        rng.gen_range(4..=6).to_string()
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // modified from rosetta code
        // im not lazy you are!
        let mut triangle = vec!["▲".into()];
        for i in 0..rng.gen_range(4..=6) {
            let space = " ".repeat(2usize.pow(i));
            let mut working = triangle.clone();

            working.iter_mut().for_each(|r| {
                let n = format!("{space}{r}{space}");
                *r = n
            });

            triangle.iter().for_each(|r| {
                let n = format!("{r} {r}");
                working.push(n)
            });

            triangle = working;
        }

        triangle
            .iter()
            .map(|x| x.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Problem, SierpinskiTriangle};
    use rand::RngCore;

    #[test]
    fn nth_prime() {
        let seed = rand::thread_rng().next_u64();
        let tris = SierpinskiTriangle.gen(seed);

        let mut triangle = vec!["▲".into()];
        for i in 0..tris.parse().unwrap() {
            let space = " ".repeat(2usize.pow(i));
            let mut working = triangle.clone();

            working.iter_mut().for_each(|r| {
                let n = format!("{space}{r}{space}");
                *r = n
            });

            triangle.iter().for_each(|r| {
                let n = format!("{r} {r}");
                working.push(n)
            });

            triangle = working;
        }

        assert_eq!(
            triangle
                .iter()
                .map(|x| x.trim_end())
                .collect::<Vec<_>>()
                .join("\n"),
            SierpinskiTriangle.check(seed)
        );
    }
}
