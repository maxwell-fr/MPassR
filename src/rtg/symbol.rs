use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;
use crate::rtg::default_lists::get_ez_ascii_symbols;

/// Maintain a list of symbols and return one as a token upon request.
pub struct SymbolsRTG {
    token_list: Vec<String>
}

impl SymbolsRTG {
}

impl RandomTokenGenerator for SymbolsRTG {
    fn new() -> Self {
        Self::with_token_list(get_ez_ascii_symbols())
    }

    fn with_token_list(token_list: Vec<impl ToString>) -> Self {
        SymbolsRTG {
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

impl Display for SymbolsRTG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "symbol({})", self.token_list.len())

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtg() {
        let input_list = vec!["#", "@", "!"];
        let output_list = vec!["#", "@", "!"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(SymbolsRTG::with_token_list(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}