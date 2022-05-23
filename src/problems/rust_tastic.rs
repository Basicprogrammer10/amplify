use super::Problem;

pub struct RustTastic;

impl Problem for RustTastic {
    fn id(&self) -> u64 {
        16
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Rust Tastic"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, _: u64) -> String {
        String::new()
    }

    fn check(&self, _: u64, output: String) -> bool {
       output == "10"
    }
}

#[cfg(test)]
mod test {
    use super::{RustTastic, Problem};

    #[test]
    fn rust_tastic() {
        assert!(RustTastic.check(0, "10".to_owned()))
    }
}
