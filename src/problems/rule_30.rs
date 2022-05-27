use super::Problem;

pub struct Rule30;

impl Problem for Rule30 {
    fn id(&self) -> u64 {
        11
    }

    fn time(&self) -> u64 {
        1653273796
    }

    fn name(&self) -> &'static str {
        "Rule 30"
    }

    fn text(&self) -> &'static str {
        "todo"
    }

    fn gen(&self, _: u64) -> String {
        String::new()
    }

    fn check(&self, _: u64) -> String {
        let mut out = Vec::new();

        let mut world = [false; 101];
        world[50] = true;

        for _ in 0..50 {
            out.push(
                world
                    .iter()
                    .map(|x| if *x { "█" } else { " " })
                    .collect::<String>()
                    .trim_end()
                    .to_owned(),
            );

            let s = world;
            for i in 0..100 {
                world[i] = match (s[i.saturating_sub(1)], s[i], s[i + 1]) {
                    (true, false, false)
                    | (false, true, true)
                    | (false, true, false)
                    | (false, false, true) => true,
                    _ => false,
                };
            }
        }

        out.join("\n")
    }
}

#[cfg(test)]
mod test {
    use crate::problems::{rule_30::Rule30, Problem};

    #[test]
    fn rule_30() {
        let mut out = Vec::new();

        let mut world = [false; 101];
        world[50] = true;

        for _ in 0..50 {
            out.push(
                world
                    .iter()
                    .map(|x| if *x { "█" } else { " " })
                    .collect::<String>()
                    .trim_end()
                    .to_owned(),
            );

            let s = world;
            for i in 0..100 {
                world[i] = match (s[i.saturating_sub(1)], s[i], s[i + 1]) {
                    (true, false, false)
                    | (false, true, true)
                    | (false, true, false)
                    | (false, false, true) => true,
                    _ => false,
                };
            }
        }

        assert_eq!(Rule30.check(0), out.join("\n"));
    }
}
