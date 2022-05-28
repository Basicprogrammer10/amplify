use std::ops::Range;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct PositionOrders;

impl Problem for PositionOrders {
    fn id(&self) -> u64 {
        3
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Position Orders"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/position_orders.html")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..rng.gen_range(10..20) {
            let mut section = Vec::new();
            for _ in 0..rng.gen_range(10..20) {
                section.push(["up", "down", "left", "right"][rng.gen_range(0..4) as usize]);
            }
            out.push(section.join(","));
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..rng.gen_range::<i32, Range<i32>>(10..20) {
            let mut pos = (0, 0);
            for _ in 0..rng.gen_range::<i32, Range<i32>>(10..20) {
                match rng.gen_range::<i32, Range<i32>>(0..4) {
                    0 => pos.0 += 1,
                    1 => pos.0 -= 1,
                    2 => pos.1 -= 1,
                    3 => pos.1 += 1,
                    _ => unreachable!(),
                }
            }
            out.push(format!("({},{})", pos.0, pos.1));
        }

        out.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{PositionOrders, Problem};
    use rand::RngCore;

    #[test]
    fn bracket_depth() {
        let seed = rand::thread_rng().next_u64();
        let pos = PositionOrders.gen(seed);
        let mut out = Vec::new();

        for i in pos.split(" ") {
            let mut slice_pos = (0, 0);
            for j in i.split(",") {
                match j {
                    "up" => slice_pos.0 += 1,
                    "down" => slice_pos.0 -= 1,
                    "left" => slice_pos.1 -= 1,
                    "right" => slice_pos.1 += 1,
                    _ => unreachable!(),
                }
            }
            out.push(format!("({},{})", slice_pos.0, slice_pos.1));
        }

        assert_eq!(PositionOrders.check(seed), out.join("\n"));
    }
}
