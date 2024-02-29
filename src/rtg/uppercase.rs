use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

pub struct UppercaseWordsRTG {
    token_list: Vec<String>,
}

impl UppercaseWordsRTG {
}

impl RandomTokenGenerator for UppercaseWordsRTG {
    fn new(token_list: Vec<impl ToString>) -> Self {
        UppercaseWordsRTG {
            token_list: token_list.iter()
                                  .map(|t| t.to_string().to_ascii_uppercase())
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
        let input_list = vec!["foo", "bar", "baz"];
        let output_list = vec!["FOO", "BAR", "BAZ"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(UppercaseWordsRTG::new(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}