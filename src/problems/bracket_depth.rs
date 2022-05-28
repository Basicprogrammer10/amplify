use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct BracketDepth;

impl Problem for BracketDepth {
    fn id(&self) -> u64 {
        2
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Bracket Depth"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/bracket_depth.html")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut fixdepth = 0;
            let mut sub = String::new();

            for _ in 0..rng.gen_range(10..20) {
                if rng.gen_bool(0.5) {
                    sub.push('{');
                    fixdepth += 1;
                    continue;
                }

                if fixdepth == 0 {
                    continue;
                }
                let step_back = rng.gen_range(0..=fixdepth);
                fixdepth -= step_back;
                sub.push_str(&"}".repeat(step_back));
            }

            sub.push_str(&"}".repeat(fixdepth));
            out.push(sub);
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut fixdepth = 0usize;
            let mut max_depth = 0;

            for _ in 0..rng.gen_range(10..20) {
                if rng.gen_bool(0.5) {
                    fixdepth += 1;
                    max_depth = max_depth.max(fixdepth);
                    continue;
                }

                if fixdepth == 0 {
                    continue;
                }
                let step_back = rng.gen_range(0..=fixdepth);
                fixdepth -= step_back;
            }

            out.push(max_depth.to_string());
        }

        out.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{BracketDepth, Problem};
    use rand::RngCore;

    #[test]
    fn bracket_depth() {
        // let seed = rand::thread_rng().next_u64();
        let seed = 50306817;
        let brackets = BracketDepth.gen(seed);
        let mut out = Vec::new();

        for set in brackets.split(" ") {
            let mut depth = 0;
            let mut max_depth = 0;

            for i in set.chars() {
                match i {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => unreachable!(),
                }
                max_depth = max_depth.max(depth);
            }

            out.push(max_depth.to_string());
        }

        print!("{}", brackets);
        assert_eq!(BracketDepth.check(seed), out.join("\n"));
    }
}
