use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

const PEICES: [char; 3] = ['_', 'X', 'O'];

pub struct TicTacToe;

impl Problem for TicTacToe {
    fn id(&self) -> u64 {
        5
    }

    fn time(&self) -> u64 {
        1654488000
    }

    fn name(&self) -> &'static str {
        "Tic Tac Toe"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/tic_tac_toe")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            out.push(format!(
                "[{}]",
                (0..3)
                    .map(|_| {
                        format!(
                            "{}{}{}",
                            PEICES[rng.gen_range(0..3) as usize],
                            PEICES[rng.gen_range(0..3) as usize],
                            PEICES[rng.gen_range(0..3) as usize]
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }

        format!("[{}]", out.join(","))
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // function? whats that!
        let bords = (0..90)
            .map(|_| [Token::None, Token::Ex, Token::Oh][rng.gen_range(0..3) as usize])
            .collect::<Vec<_>>()
            .chunks(3)
            .map(Vec::from)
            .collect::<Vec<_>>()
            .chunks(3)
            .map(Vec::from)
            .collect::<Vec<_>>();

        score_arr(bords)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    Ex,
    Oh,
    None,
}

fn score_arr(bords: Vec<Vec<Vec<Token>>>) -> String {
    let mut score = Vec::new();

    'b: for b in bords {
        // Check Hor
        for i in b.iter() {
            if i[0] == i[1] && i[1] == i[2] && i[2] != Token::None {
                score.push("H");
                continue 'b;
            }
        }

        // Check Ver
        for i in 0..3 {
            if b[0][i] == b[1][i] && b[1][i] == b[2][i] && b[2][i] != Token::None {
                score.push("V");
                continue 'b;
            }
        }

        // Check Diag
        if b[1][1] != Token::None && (b[0][0] == b[1][1] && b[1][1] == b[2][2])
            || (b[0][2] == b[1][1] && b[1][1] == b[2][0])
        {
            score.push("D");
            continue 'b;
        }

        score.push("_");
    }

    format!("[{}]", score.join(","))
}

#[cfg(test)]
mod test {
    use super::{score_arr, Problem, TicTacToe, Token};
    use rand::RngCore;

    #[test]
    fn nth_prime() {
        let seed = rand::thread_rng().next_u64();
        let toes = TicTacToe.gen(seed);
        println!("{toes}");

        // TOKENIZE *goddam*
        let seg = toes.split(",").collect::<Vec<_>>();
        let mut lines = Vec::new();

        for i in seg {
            let chars = i.chars().collect::<Vec<_>>();
            let mut line = Vec::new();
            let mut i = 0;

            while i < chars.len() {
                match chars[i] {
                    'X' => line.push(Token::Ex),
                    'O' => line.push(Token::Oh),
                    '_' => line.push(Token::None),
                    _ => {}
                }

                i += 1;
            }

            lines.push(line);
        }

        // Score bords
        let bords = lines
            .chunks(3)
            .map(|x| {
                let mut v = Vec::new();
                v.extend(x.iter().map(|x| x.to_owned()));
                v
            })
            .collect::<Vec<_>>();

        assert_eq!(TicTacToe.check(seed), score_arr(bords));
    }
}
