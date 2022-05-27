use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

const DURATION_CONVERSION: [(&str, u32); 5] = [
    ("second", 60),
    ("minute", 60),
    ("hour", 24),
    ("day", 7),
    ("weak", 0),
];

pub struct ReadableDuration;

impl Problem for ReadableDuration {
    fn id(&self) -> u64 {
        10
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Readable Duration"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        (0..10)
            .map(|_| rng.gen_range(0..100000000).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        (0..10)
            .map(|_| {
                let mut i = rng.gen_range(0..100000000) as f32;

                for j in DURATION_CONVERSION {
                    if j.1 == 0 || i < j.1 as f32 {
                        i = i.round();
                        return format!("{} {}{}", i, j.0, if i > 1. { "s" } else { "" });
                    }

                    i /= j.1 as f32;
                }

                format!("{} weaks", i.round())
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Problem, ReadableDuration, DURATION_CONVERSION};
    use rand::RngCore;

    #[test]
    fn readable_duration() {
        let seed = rand::thread_rng().next_u64();
        let durations = ReadableDuration.gen(seed);
        let mut out = Vec::new();

        'm: for i in durations.split(" ").map(|x| x.parse::<u32>().unwrap()) {
            let mut i = i as f32;

            for j in DURATION_CONVERSION {
                if j.1 == 0 || i < j.1 as f32 {
                    i = i.round();
                    out.push(format!("{} {}{}", i, j.0, if i > 1. { "s" } else { "" }));
                    continue 'm;
                }

                i /= j.1 as f32;
            }

            out.push(format!("{} weaks", i.round()));
        }

        assert_eq!(ReadableDuration.check(seed), out.join("\n"));
    }
}
