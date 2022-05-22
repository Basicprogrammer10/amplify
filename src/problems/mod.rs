mod simple_math;

pub trait Problem {
    // Problem ID
    fn id() -> u64;

    // Get problem text
    fn text() -> &'static str;

    // Gen the run args
    fn gen(seed: u64) -> String;

    // Check the program output
    fn check(seed: u64, output: String) -> bool;
}
