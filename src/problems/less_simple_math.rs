use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct LessSimpleMath;

impl Problem for LessSimpleMath {
    fn id(&self) -> u64 {
        16
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Less Simple Math"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();

        for _ in 0..10 {
            let mut seg = vec![rng.gen_range(1..100).to_string()];
            for _ in 0..rng.gen_range(3..10) {
                match rng.gen_range(0..4) {
                    0 => seg.push(format!("+{}", rng.gen_range(1..100))),
                    1 => seg.push(format!("-{}", rng.gen_range(1..100))),
                    2 => seg.push(format!("*{}", rng.gen_range(1..100))),
                    3 => seg.push(gen_par_seg(&mut rng)),
                    _ => unreachable!(),
                }
            }

            out.push(seg.join(""));
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        todo!()
    }
}

fn gen_par_seg(rng: &mut ChaCha8Rng) -> String {
    let mut out = rng.gen_range(1..100).to_string();

    for _ in 0..rng.gen_range(2..6) {
        match rng.gen_range(0..3) {
            0 => out.push_str(&format!("+{}", rng.gen_range(1..100))),
            1 => out.push_str(&format!("-{}", rng.gen_range(1..100))),
            2 => out.push_str(&format!("*{}", rng.gen_range(1..100))),
            _ => unreachable!(),
        }
    }

    format!("{}({out})", ["+", "-"][rng.gen_range(0..2) as usize])
}

#[cfg(test)]
mod test {
    use super::{LessSimpleMath, Problem};
    use rand::RngCore;

    const DIGIT: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    #[derive(Debug)]
    enum Token {
        Group(Vec<Token>),
        Num(i32),
        Add,
        Sub,
        Mul,
    }

    fn push_token(tokens: &mut Vec<Token>, group: &mut Option<Vec<Token>>, token: Token) {
        if let Some(i) = group {
            return i.push(token);
        }
        tokens.push(token);
    }

    // fn execute(mut tokens: Vec<Token>) {
    //     // Mul = 2
    //     // Add / Sub = 1
    //     let mut level = 0;
    //     let mut i = 0;

    //     while tokens.len() > 1 {
    //         let f = tokens.iter().enumerate().find(|x| {

    //         });
    //     }
    // }

    #[test]
    fn less_simple_math() {
        let seed = rand::thread_rng().next_u64();
        let math = LessSimpleMath.gen(seed);

        for exp in math.split(" ").take(1) {
            // Tokenize
            let mut tokens = Vec::new();
            let chars = exp.chars().collect::<Vec<_>>();
            let mut i = 0;

            let mut parse_num = false;
            let mut num_build = String::new();
            let mut group = None;

            while i < chars.len() {
                let on_digit = DIGIT.contains(&chars[i]);
                if on_digit {
                    parse_num = true;
                    num_build.push(chars[i]);
                }
                if !on_digit && parse_num {
                    parse_num = false;
                    push_token(
                        &mut tokens,
                        &mut group,
                        Token::Num(num_build.parse::<i32>().unwrap()),
                    );
                    num_build.clear();
                }

                match chars[i] {
                    '+' => push_token(&mut tokens, &mut group, Token::Add),
                    '-' => push_token(&mut tokens, &mut group, Token::Sub),
                    '*' => push_token(&mut tokens, &mut group, Token::Mul),
                    '(' => group = Some(Vec::new()),
                    ')' => {
                        tokens.push(Token::Group(group.unwrap()));
                        group = None;
                    }

                    _ => {}
                }

                i += 1;
            }

            // Execute parentasies
            // *ALL OF THE BODGES*

            for i in &mut tokens {
                if let Token::Group(t) = i {}
            }

            println!("{}", exp);
            dbg!(tokens);
        }
    }
}
