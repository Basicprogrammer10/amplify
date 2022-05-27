use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct PokerScorer;

impl Problem for PokerScorer {
    fn id(&self) -> u64 {
        7
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Poker Scorer"
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{PokerScorer, Problem};
    use rand::RngCore;

    #[test]
    fn simple_math() {
        // let seed = rand::thread_rng().next_u64();
        let seed = 12;
        let tags = PokerScorer.gen(seed);
    }
}
