use std::{cmp::Reverse, vec};

use lazy_static::lazy_static;

mod bracket_depth;
mod increase_count;
mod nth_prine;
mod rust_tastic;
mod simple_math;

lazy_static! {
    pub static ref PROBLEMS: Vec<Box<dyn Problem + Send + Sync>> = {
        let mut problems: Vec<Box<dyn Problem + Send + Sync>> = vec![
            Box::new(bracket_depth::BracketDepth),
            Box::new(nth_prine::NthPrime),
            Box::new(rust_tastic::RustTastic),
            Box::new(simple_math::SimpleMath),
            Box::new(increase_count::IncreaseCount),
        ];
        problems.sort_by_key(|b| Reverse(b.id()));
        problems
    };
}

pub trait Problem {
    // Problem ID
    fn id(&self) -> u64;

    // Problem actavation time
    fn time(&self) -> u64;

    // Problem name
    fn name(&self) -> &'static str;

    // Get problem text
    fn text(&self) -> &'static str;

    // Gen the run args
    fn gen(&self, seed: u64) -> String;

    // Check the program output
    fn check(&self, seed: u64) -> String;
}
