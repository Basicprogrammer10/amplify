use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

pub struct DurationParser;

impl Problem for DurationParser {
    fn id(&self) -> u64 {
        10
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Duration Parser"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        "1 hour 3 days 10 seconds 9 minutes".to_owned()
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::{DurationParser, Problem};
    use rand::RngCore;

    const DIGIT: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    #[derive(Debug)]
    enum Token {
        Num(u32),
        Second,
        Minute,
        Hour,
        Day,
    }

    #[test]
    fn simple_math() {
        let seed = rand::thread_rng().next_u64();
        let time = DurationParser.gen(seed);

        // Tokenize (sorry this is bad)
        let chars = time.chars().collect::<Vec<_>>();
        let mut tokens = Vec::new();
        let mut i = 0;

        let mut parse_num = false;
        let mut num_builder = String::new();

        let mut parse_ident = false;
        let mut ident_builder = String::new();

        while i < chars.len() {
            let is_digit = DIGIT.contains(&chars[i]);

            if is_digit {
                parse_num = true;
                num_builder.push(chars[i]);
            }

            if !is_digit && parse_num {
                parse_num = false;
                tokens.push(Token::Num(num_builder.parse().unwrap()));
                num_builder.clear();
            }

            if parse_ident && (is_digit || i == chars.len() - 1) {
                parse_ident = false;
                tokens.push(
                    match ident_builder.chars().next().unwrap().to_ascii_lowercase() {
                        's' => Token::Second,
                        'm' => Token::Minute,
                        'h' => Token::Hour,
                        'd' => Token::Day,
                        _ => panic!(),
                    },
                );
                ident_builder.clear();
            }

            if !is_digit && !chars[i].is_whitespace() {
                parse_ident = true;
                ident_builder.push(chars[i]);
            }

            i += 1;
        }

        // Expand
        let mut current = 0;
        let mut total = 0;
        let mut i = 0;

        while i < tokens.len() {
            match tokens[i] {
                Token::Num(i) => current = i,
                Token::Second => total += current,
                Token::Minute => total += current * 60,
                Token::Hour => total += current * (60 * 60),
                Token::Day => total += current * (60 * 60 * 24),
            }
            i += 1;
        }

        println!("{} secs", total);
    }
}
