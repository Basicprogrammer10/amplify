use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn rand_str(rng: &mut ChaCha8Rng) -> String {
    (0..rng.gen_range(10..20))
        .map(|_| rng.gen_range(64..=126) as u8 as char)
        .collect::<String>()
}
