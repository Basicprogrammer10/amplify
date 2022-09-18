use super::Problem;

const PROBLEM: [(&str, u8); 4] = [
    ("[[__________],[__________],[___XX_XX__],[__X_XX__X_],[__XX____X_],[___X___X__],[__X____XX_],[__X__XX_X_],[___XX_XX__],[__________]]", 5),
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
    use std::vec;

    use super::{ConwaysGameOfLife, Problem};
    use rand::RngCore;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Cell {
        Live,
        Dead,
    }

    #[test]
    fn game_of_life() {
        let seed = rand::thread_rng().next_u64();
        let mut boards = Vec::new();

        // Tokenize to Bords
        let mut bord_builder = Vec::new();
        let mut builder = Vec::new();
        let mut iter_builder = String::new();
        for i in ConwaysGameOfLife.gen(seed).chars() {
            if i.is_ascii_digit() {
                iter_builder.push(i);
            }

            match i {
                'X' => builder.push(Cell::Live),
                '_' => builder.push(Cell::Dead),
                ',' => {
                    bord_builder.push(builder);
                    builder = Vec::new();
                }
                ')' => {
                    boards.push((bord_builder, iter_builder.parse::<usize>().unwrap()));
                    bord_builder = Vec::new();
                    iter_builder = String::new();
                }
                _ => {}
            }
        }

        // For each board, simulate n times and print
        let mut buff = vec![vec![Cell::Dead; 10]; 10];
        let mut out = Vec::new();
        for (mut cells, n) in boards {
            // Sim board for each required step
            for _ in 0..n {
                // quick and dirty,,, just how i like it 😈
                for x in 0..10_usize {
                    for y in 0..10_usize {
                        // Count neighbours
                        let mut neighbours = 0;

                        for x1 in x.saturating_sub(1)..=x + 1 {
                            for y1 in y.saturating_sub(1)..=y + 1 {
                                if x1 > 9 || y1 > 9 || (x1 == x && y1 == y) {
                                    continue;
                                }

                                if cells[y1][x1] == Cell::Live {
                                    neighbours += 1;
                                }
                            }
                        }

                        let old_state = cells[y][x];
                        let mut new_state = Cell::Dead;

                        if (old_state == Cell::Live && matches!(neighbours, 2 | 3))
                            || (old_state == Cell::Dead && neighbours == 3)
                        {
                            new_state = Cell::Live;
                        }

                        buff[y][x] = new_state;
                    }
                }

                cells = buff;
                buff = vec![vec![Cell::Dead; 10]; 10];
            }

            out.push(cells);
        }

        // Print simulated bords
        let mut str_out = Vec::new();
        for i in out {
            let mut rows = Vec::new();
            for y in i {
                let row = y
                    .iter()
                    .map(|x| match x {
                        Cell::Dead => '_',
                        Cell::Live => 'X',
                    })
                    .collect::<String>();
                rows.push(format!("[{}]", row));
            }
            str_out.push(format!("[{}]", rows.join(",")));
        }

        assert_eq!(ConwaysGameOfLife.check(seed), str_out.join("\n"));
    }
}
