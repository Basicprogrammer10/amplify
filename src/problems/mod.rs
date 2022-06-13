use std::{cmp::Reverse, vec};

use lazy_static::lazy_static;

mod base_64_decode;
mod bracket_depth;
mod brain_f;
mod common;
mod duration_parser;
mod increase_count;
mod less_simple_math; //
mod nth_prime;
mod position_orders;
mod readable_duration;
mod rule_30;
mod rust_tastic;
mod sierpinski_triangle;
mod simple_math;
mod tag_verifier;
mod tic_tac_toe;
mod url_decode;

lazy_static! {
    pub static ref PROBLEMS: Vec<Box<dyn Problem + Send + Sync>> = {
        let mut problems: Vec<Box<dyn Problem + Send + Sync>> = vec![
            Box::new(base_64_decode::Base64Decode),
            Box::new(bracket_depth::BracketDepth),
            Box::new(brain_f::BrainF),
            Box::new(duration_parser::DurationParser),
            Box::new(nth_prime::NthPrime),
            Box::new(rust_tastic::RustTastic),
            Box::new(position_orders::PositionOrders),
            Box::new(readable_duration::ReadableDuration),
            Box::new(rule_30::Rule30),
            Box::new(sierpinski_triangle::SierpinskiTriangle),
            Box::new(simple_math::SimpleMath),
            Box::new(increase_count::IncreaseCount),
            Box::new(tag_verifier::TagVerifier),
            Box::new(tic_tac_toe::TicTacToe),
            Box::new(url_decode::UrlDecode),
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
