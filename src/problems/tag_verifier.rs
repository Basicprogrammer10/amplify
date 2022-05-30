use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::Problem;

const TAGS: [&str; 19] = [
    "article", "h1", "h2", "h3", "h4", "h5", "h6", "p", "ul", "ol", "li", "p", "pre", "code", "a",
    "em", "strong", "sub", "sup",
];

pub struct TagVerifier;

impl Problem for TagVerifier {
    fn id(&self) -> u64 {
        6
    }

    fn time(&self) -> u64 {
        1654574400
    }

    fn name(&self) -> &'static str {
        "Tag Verifier"
    }

    fn text(&self) -> &'static str {
        include_str!("./text/build/tag_verifier")
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut out = Vec::new();
        let ans = (0..10).map(|_| rng.gen_bool(0.5)).collect::<Vec<_>>();

        for i in 0..10 {
            let mut open = Vec::new();
            let mut seg = Vec::new();

            for _ in 0..rng.gen_range(5..10) {
                if rng.gen_bool(0.5) {
                    let tag = TAGS[rng.gen_range(0..19) as usize];
                    seg.push(format!("<{tag}>"));
                    open.push(tag);
                    continue;
                }

                if open.len() <= 1 {
                    continue;
                }

                for _ in 0..rng.gen_range(1..open.len()) {
                    seg.push(format!("</{}>", open.pop().unwrap()));
                }
            }

            seg.extend(open.iter().rev().map(|x| format!("</{x}>")));

            if !ans[i] {
                for _ in 0..rng.gen_range(1..2) {
                    if seg.is_empty() {
                        break;
                    }

                    seg.remove(rng.gen_range(0..seg.len()));
                }
            }

            out.push(seg.join(""))
        }

        out.join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .map(|_| rng.gen_bool(0.5).to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{Problem, TagVerifier};
    use rand::RngCore;

    #[derive(Debug)]
    enum Token {
        Start(String),
        End(String),
    }

    #[test]
    fn tag_verifier() {
        let seed = rand::thread_rng().next_u64();
        let tags = TagVerifier.gen(seed);
        let mut out = Vec::new();

        for line in tags.split(" ") {
            // Tokenize
            let mut i = 0;
            let chars = line.chars().collect::<Vec<_>>();
            let mut tokens = Vec::new();

            let mut parse_tag = false;
            let mut tag_type = false; // 0 OPEN -- 1 CLOSED
            let mut tag_builder = String::new();

            while i < chars.len() {
                match chars[i] {
                    '<' => parse_tag = true,
                    '>' => {
                        parse_tag = false;
                        tokens.push(match tag_type {
                            true => Token::End(tag_builder.to_owned()),
                            false => Token::Start(tag_builder.to_owned()),
                        });
                        tag_type = false;
                        tag_builder.clear();
                    }
                    '/' => tag_type = true,
                    _ => {
                        if parse_tag {
                            tag_builder.push(chars[i]);
                        }
                    }
                }

                i += 1;
            }

            // Check
            let mut i = 0;
            let mut ok = true;
            let mut stack = HashMap::<String, Vec<usize>>::new();

            while i < tokens.len() {
                match &tokens[i] {
                    Token::Start(e) => {
                        let arr = stack.entry(e.to_owned()).or_default();
                        if !arr.contains(&i) {
                            arr.push(i);
                        }
                    }
                    Token::End(e) => {
                        if let Some(oi) = stack.entry(e.to_owned()).or_default().pop() {
                            tokens.remove(i);
                            tokens.remove(oi);
                            i = 0;
                            continue;
                        }

                        ok = false;
                        break;
                    }
                }

                i += 1;
            }

            ok = ok && !stack.iter().fold(false, |inc, x| inc || x.1.len() > 0);
            out.push(ok.to_string());
        }

        assert_eq!(TagVerifier.check(seed), out.join("\n"));
    }
}
