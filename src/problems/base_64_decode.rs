use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::{common::rand_str, Problem};

pub struct Base64Decode;

impl Problem for Base64Decode {
    fn id(&self) -> u64 {
        12
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Base64 Decode"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .map(|_| base64::encode(rand_str(&mut rng)))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .map(|_| rand_str(&mut rng))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Base64Decode, Problem};

    #[test]
    fn base_64_decode() {
        let seed = 12;
        let code = Base64Decode.gen(seed);
        let mut out = Vec::new();

        for i in code.split(" ") {
            let result = i
                .chars()
                .filter(|&ch| ch != '=')
                .map(|ch| {
                    let ascii = ch as i8;
                    let convert = match ch {
                        '0'..='9' => ascii + 4,
                        'a'..='z' => ascii + -71,
                        'A'..='Z' => ascii + -65,
                        '+' => 62,
                        '/' => 63,
                        _ => unreachable!(),
                    };
                    format!("{:#08b}", convert)[2..].to_owned()
                })
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>()
                .chunks(8)
                .map(|chunk| {
                    let num_str = chunk.iter().collect::<String>();
                    usize::from_str_radix(&num_str, 2).unwrap() as u8
                })
                .filter(|x| *x != 0)
                .collect::<Vec<_>>();

            out.push(String::from_utf8(result).unwrap());
        }

        assert_eq!(out.join("\n"), Base64Decode.check(seed));
    }
}
