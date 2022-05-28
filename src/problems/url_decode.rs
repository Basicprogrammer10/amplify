use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use super::{common::rand_str, Problem};

pub struct UrlDecode;

impl Problem for UrlDecode {
    fn id(&self) -> u64 {
        7
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Url Decode"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .map(|_| urlencoding::encode(&rand_str(&mut rng)).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn check(&self, seed: u64) -> String {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        (0..10)
            .map(|_| rand_str(&mut rng))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Problem, UrlDecode};
    use afire::internal::common::decode_url;
    use rand::RngCore;

    #[test]
    fn url_decode() {
        let seed = rand::thread_rng().next_u64();
        let url = UrlDecode.gen(seed);
        let mut out = Vec::new();

        for i in url.split(" ") {
            out.push(decode_url(i.to_owned()));
        }

        assert_eq!(out.join("\n"), UrlDecode.check(seed));
    }
}
