pub trait Problem {
    fn gen() -> (String, String);
    fn check(seed: String) -> bool;
}
