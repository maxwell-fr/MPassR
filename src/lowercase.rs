use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

pub struct LowercaseWordsRTG {
    token_list: Vec<String>,
}

impl LowercaseWordsRTG {
}

impl RandomTokenGenerator for LowercaseWordsRTG {
    fn new(token_list: Vec<impl ToString>) -> Self {
        LowercaseWordsRTG {
            token_list: token_list.iter()
                                  .map(|t| t.to_string().to_ascii_lowercase())
                                  .collect(),
        }
    }

    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let tok = self.token_list.get(idx).unwrap_or(&String::from("")).clone();
        tok
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtg() {
        let input_list = vec!["FOO", "BAR", "BAZ"];
        let output_list = vec!["foo", "bar", "baz"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(LowercaseWordsRTG::new(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}