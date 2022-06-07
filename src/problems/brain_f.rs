use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

const PROBLEMS: [&str; 4] = [
    "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.",
    ">>>>>>>>>>>>>>>>>>>>>>>>>>>>++++++++++++++++++++++++++[-<<[+<]+[>]>][<<[[-]-----<]>[>]>]<<[++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<]>[.>]",
    "+++++++++++++[>+++++<-]>.>++++[<+++>-]<.+++.----.---.---.>++++[<+++++>-]<-.",
    "++++++++++[>+++++++<-]>+.>++++++++++[>++++++++<-]>-.>++++[>++++++++<-]>.<<<<.>>.>>.<<--.<+++[>----<-]>.<++++[>+++<-]>+.<<.>>+."
];
const SOLUTIONS: [&str; 4] = [
    "Hello World!",
    "abcdefghijklmnopqrstuvwxyz",
    "AMPLIFY",
    "GO GO MANGO",
];

pub struct BrainF;

impl Problem for BrainF {
    fn id(&self) -> u64 {
        13
    }

    fn time(&self) -> u64 {
        0
    }

    fn name(&self) -> &'static str {
        "Brain F"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut prbs = PROBLEMS.to_owned();
        prbs.shuffle(&mut rng);
        prbs.join(",")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut prbs = SOLUTIONS.to_owned();
        prbs.shuffle(&mut rng);
        prbs.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{BrainF, Problem};
    use rand::RngCore;

    #[test]
    fn brain_f() {
        let seed = rand::thread_rng().next_u64();
        let inp = BrainF.gen(seed);
        let mut out = Vec::new();

        // from my code golf soluion,,,
        for i in inp.split(",") {
            let code = i.chars().collect::<Vec<_>>();
            let mut pout = String::new();
            let mut data = [0; 99];
            let mut index = 0;
            let mut pgrm = 0;
            let mut inc;

            'main: loop {
                if pgrm >= code.len() {
                    break;
                };

                match code[pgrm] {
                    '>' => index += 1,
                    '<' => index -= 1,
                    '+' => data[index] += 1,
                    '-' => data[index] -= 1,
                    '.' => pout.push(data[index] as u8 as char),
                    '[' => {
                        inc = 0;
                        if data[index] != 0 {
                            pgrm += 1;
                            continue;
                        }

                        for (j, x) in code.iter().enumerate().skip(pgrm) {
                            match x {
                                '[' => inc += 1,
                                ']' => {
                                    inc -= 1;
                                    if inc == 0 {
                                        pgrm = j;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    ']' => {
                        inc = 0;
                        for (j, x) in code.iter().enumerate().rev().skip(code.len() - pgrm) {
                            match x {
                                '[' => {
                                    if inc < 1 {
                                        pgrm = j;
                                        continue 'main;
                                    }
                                    inc -= 1
                                }
                                ']' => inc += 1,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
                pgrm += 1
            }

            out.push(pout);
        }

        assert_eq!(out.join("\n"), BrainF.check(seed))
    }
}
