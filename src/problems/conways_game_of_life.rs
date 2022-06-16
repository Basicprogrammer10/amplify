use super::Problem;

const PROBLEM: [(&str, u8); 4] = [
    ("[[__________],[__________],[___XX_XX__],[__X_XX__X_],[__XX____X_],[___X___X__],[__X____XX_],[__X__XX_X_],[___XX_XX__],[__________]]", 6),
    ("[[X_X_______],[_XX_______],[_X________],[__________],[__________],[__________],[__________],[__________],[__________],[__________]]", 16),
    ("[[__________],[____XX____],[___X__X___],[__X____X__],[_X______X_],[_X______X_],[__X____X__],[___X__X___],[____XX____],[__________]]", 7),
    ("[[__________],[_____X____],[___X_X____],[__X____X__],[__XXXXXX__],[__________],[____XX____],[____XX____],[__________],[__________]]", 15)
];
const SOLUTION: [&str; 4] = [
    "[[__________],[__________],[___XX_XX__],[__X__XX_X_],[__X____XX_],[___X___X__],[__XX____X_],[__X_XX__X_],[___XX_XX__],[__________]]",
    "[[__________],[__________],[__________],[__________],[____X_X___],[_____XX___],[_____X____],[__________],[__________],[__________]]",
    "[[__________],[___X__X___],[___X__X___],[_XX_XX_XX_],[___X__X___],[___X__X___],[_XX_XX_XX_],[___X__X___],[___X__X___],[__________]]",
    "[[__________],[____X_____],[____X_X___],[__X____X__],[__XXXXXX__],[__________],[____XX____],[____XX____],[__________],[__________]]"
];

pub struct ConwaysGameOfLife;

impl Problem for ConwaysGameOfLife {
    fn id(&self) -> u64 {
        14
    }

    fn time(&self) -> u64 {
        1655265600
    }

    fn name(&self) -> &'static str {
        "Conway's Game of Life"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/conways_game_of_life")
    }

    fn gen(&self, _seed: u64) -> String {
        format!(
            "[{}]",
            PROBLEM.map(|(a, b)| format!("({},{})", a, b)).join(" ")
        )
    }

    fn check(&self, _seed: u64) -> String {
        SOLUTION.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{ConwaysGameOfLife, Problem};
    use rand::RngCore;

    #[test]
    fn game_of_life() {
        let seed = rand::thread_rng().next_u64();
        println!("{}", ConwaysGameOfLife.gen(12));
        println!();
        println!("{}", ConwaysGameOfLife.check(12));
    }
}
