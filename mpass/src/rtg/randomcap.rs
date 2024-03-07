//! [`RTG`] Struct

use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

/// A random token generator that produces a word
/// with a single letter capitalized.
pub struct RandomCapWordGenerator {
    token_list: Vec<String>
}

impl RandomCapWordGenerator {
    // Create a new RandomCapitalWord from a Vec of anything that implements to_string().
    pub fn new(token_list: Vec<impl ToString>) -> Self {
        RandomCapWordGenerator {
            token_list: token_list.iter()
                                  .map(|t| t.to_string().to_ascii_lowercase())
                                  .collect()
        }
    }
}

impl From<Vec<String>> for RandomCapWordGenerator {
    // Convert a Vec<String> into an RTG.
    fn from(value: Vec<String>) -> Self {
        RandomCapWordGenerator {
            token_list: value
        }
    }
}

impl RandomTokenGenerator for RandomCapWordGenerator {
    fn get_token(&self) -> String {
        let word_idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let mut tok_ch: Vec<char> = self.token_list.get(word_idx).unwrap_or(&String::from("_")).clone()
                                 .chars().collect();

        let char_idx: usize = thread_rng().gen_range(0 .. tok_ch.len());
        tok_ch[char_idx] = tok_ch[char_idx].to_ascii_uppercase();

        tok_ch.into_iter().collect()
    }
}


impl Display for RandomCapWordGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RandomCap({})", self.token_list.len())

    }
}
