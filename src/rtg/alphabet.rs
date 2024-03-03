use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

/// Maintain a list of numbers and return one as a token upon request.
pub struct AlphabetRTG {
    token_list: Vec<String>
}

impl AlphabetRTG {
}

impl RandomTokenGenerator for AlphabetRTG {
    fn new() -> Self {
        Self::with_token_list(vec!['a','b','c','d','e','f','g','h','i','j','k','l','m',
                                   'n','o','p','q','r','s','t','u','v','w','x','y','z',
                                   'A','B','C','D','E','F','G','H','I','J','K','L','M',
                                   'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'])
    }

    fn with_token_list(token_list: Vec<impl ToString>) -> Self {
        AlphabetRTG {
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

impl Display for AlphabetRTG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "alphabet({})", self.token_list.len())

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtg() {
        let input_list = vec!["a", "b", "c"];
        let output_list = vec!["a", "b", "c"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(AlphabetRTG::with_token_list(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}