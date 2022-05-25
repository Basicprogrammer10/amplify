use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

const TAGS: [&str; 19] = [
    "article", "h1", "h2", "h3", "h4", "h5", "h6", "p", "ul", "ol", "li", "p", "pre", "code", "a",
    "em", "strong", "sub", "sup",
];

pub struct TagVerifier;

impl Problem for TagVerifier {
    fn id(&self) -> u64 {
        6
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Tag Verifier"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut open = Vec::new();
            let mut seg = Vec::new();

            for _ in 0..rng.gen_range(5..10) {
                match rng.gen_bool(0.5) {
                    true => {
                        let tag = TAGS[rng.gen_range(0..19) as usize];
                        seg.push(tag);
                        open.push(tag);
                    },
                    false => {}
                }
            }

            out.push(seg.join(""))
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::{Problem, TagVerifier};
    use rand::RngCore;

    #[test]
    fn simple_math() {
        let seed = rand::thread_rng().next_u64();
        let tags = TagVerifier.gen(seed);

        println!("{}", tags);
    }
}
