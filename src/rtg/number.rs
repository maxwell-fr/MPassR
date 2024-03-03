use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

/// Maintain a list of words and return one as a token upon request.
/// The words are all lowercase.
pub struct NumbersRTG {
    token_list: Vec<String>
}

impl NumbersRTG {
}

impl RandomTokenGenerator for NumbersRTG {
    fn new() -> Self {
        Self::with_token_list(vec![0,1,2,3,4,5,6,7,8,9])
    }

    fn with_token_list(token_list: Vec<impl ToString>) -> Self {
        NumbersRTG {
            token_list: token_list.iter()
                                  .map(|t| t.to_string())
                                  .collect(),
        }
    }

    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let tok = self.token_list.get(idx).unwrap_or(&String::from("")).clone();
        tok
    }
}

impl Display for NumbersRTG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "number({})", self.token_list.len())

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtg() {
        let input_list = vec!["1", "2", "3"];
        let output_list = vec!["1", "2", "3"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(NumbersRTG::with_token_list(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}