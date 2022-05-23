use std::vec;

use lazy_static::lazy_static;

mod bracket_depth;
mod nth_prine;
mod simple_math;

lazy_static! {
    pub static ref PROBLEMS: Vec<Box<dyn Problem + Send + Sync>> = {
        let mut problems: Vec<Box<dyn Problem + Send + Sync>> = vec![
            Box::new(bracket_depth::BracketDepth),
            Box::new(nth_prine::NthPrime),
            Box::new(simple_math::SimpleMath),
        ];
        problems.sort_by(|a, b| b.id().cmp(&a.id()));
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
    fn check(&self, seed: u64, output: String) -> bool;
}
