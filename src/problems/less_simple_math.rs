use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct LessSimpleMath;

impl Problem for LessSimpleMath {
    fn id(&self) -> u64 {
        15
    }

    fn time(&self) -> u64 {
        1655352000
    }

    fn name(&self) -> &'static str {
        "Less Simple Math"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/less_simple_math")
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
                    3 => seg.push(gen_par_sec(&mut rng)),
                    _ => unreachable!(),
                }
            }

            out.push(seg.join(""));
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut out = Vec::new();
        let real = self.gen(seed);

        // TODO: gen tokens to skip tokenizing
        for i in real.split(' ') {
            out.push(evaluate(create_tree(tokenize(i))).to_string());
        }

        out.join("\n")
    }
}

fn gen_par_sec(rng: &mut ChaCha8Rng) -> String {
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

#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Op(Ops),
    Group(Vec<Token>),
    Tree(Ops, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Sub,
    Mul,
}

fn evaluate(tree: Token) -> i32 {
    match tree {
        Token::Tree(op, left, right) => {
            let left = evaluate(*left);
            let right = evaluate(*right);

            match op {
                Ops::Add => left + right,
                Ops::Sub => left - right,
                Ops::Mul => left * right,
            }
        }
        Token::Number(n) => n,
        _ => panic!("Invalid token"),
    }
}

fn create_tree(mut tokens: Vec<Token>) -> Token {
    fn get_max_prio(tokens: &[Token]) -> usize {
        tokens
            .iter()
            .filter_map(|x| match x {
                Token::Op(i) => Some(i.prio()),
                _ => None,
            })
            .max()
            .unwrap()
    }

    fn contains_non_tree(tokens: &[Token]) -> bool {
        tokens
            .iter()
            .filter(|x| !matches!(x, Token::Tree(_, _, _)))
            .count()
            > 0
    }

    while contains_non_tree(&tokens) {
        let max_prio = get_max_prio(&tokens);

        let mut i = 0;
        while i < tokens.len() {
            if let Token::Op(op) = tokens[i] {
                if op.prio() < max_prio {
                    i += 1;
                    continue;
                }

                let left = tokens.remove(i - 1).make_tree();
                let right = tokens.remove(i).make_tree();

                tokens[i - 1] = Token::Tree(op, Box::new(left), Box::new(right));
                break;
            }

            i += 1;
        }
    }

    debug_assert!(tokens.len() == 1);
    tokens[0].clone()
}

fn tokenize(inp: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut working = String::new();
    let mut in_group = false;

    for i in inp.chars() {
        if !i.is_ascii_digit() && !working.is_empty() && !in_group {
            out.push(Token::Number(working.parse().unwrap()));
            working.clear();
        }

        match i {
            // Groups
            '(' => in_group = true,
            ')' => {
                in_group = false;
                out.push(Token::Group(tokenize(&working)));
                working.clear();
            }
            i if in_group => working.push(i),

            // Operations
            '+' => out.push(Token::Op(Ops::Add)),
            '-' => out.push(Token::Op(Ops::Sub)),
            '*' => out.push(Token::Op(Ops::Mul)),

            // Numbers
            i if i.is_ascii_digit() => working.push(i),

            _ => panic!("INVALID CHAR: {}", i),
        }
    }

    if !working.is_empty() {
        out.push(Token::Number(working.parse().unwrap()));
    }

    out
}

impl Ops {
    fn prio(&self) -> usize {
        match self {
            Ops::Add | Ops::Sub => 1,
            Ops::Mul => 2,
        }
    }
}

impl Token {
    fn make_tree(self) -> Token {
        match self {
            Token::Group(tokens) => create_tree(tokens),
            _ => self,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{create_tree, evaluate, tokenize, LessSimpleMath, Problem};
    use rand::RngCore;

    #[test]
    fn less_simple_math() {
        let seed = rand::thread_rng().next_u64();
        let raw = LessSimpleMath.gen(seed);
        let mut out = Vec::new();

        for i in raw.split(" ") {
            let tokens = tokenize(&i);
            let tree = create_tree(tokens);
            let result = evaluate(tree);
            out.push(result.to_string());
        }

        assert_eq!(LessSimpleMath.check(seed), out.join("\n"));
    }
}
