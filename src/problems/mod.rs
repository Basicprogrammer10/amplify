use std::vec;

use lazy_static::lazy_static;

mod bracket_depth;
mod simple_math;

lazy_static! {
    pub static ref PROBLEMS: Vec<Box<dyn Problem + Send + Sync>> = vec![
        Box::new(simple_math::SimpleMath),
        Box::new(bracket_depth::BracketDepth)
    ];
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
