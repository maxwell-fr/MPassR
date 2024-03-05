//! [`RandomTokenGenerator`] trait
//! [`RTG`] Struct
pub mod default_lists;

use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};

/// The shared trait for all token generators.
pub trait RandomTokenGenerator : Display {
    /// Gets a token. The rules for the provided token are
    /// dependent on the trait implementor.
    fn get_token(&self) -> String;
}

/// Handy implementation of RandomTokenGenerator
pub struct RTG {
    token_list: Vec<String>
}

impl RTG {
    // Create a new RTG from a Vec of anything that implements to_string().
    pub fn new(token_list: Vec<impl ToString>) -> Self {
        RTG {
            token_list: token_list.iter()
                                  .map(|t| t.to_string())
                                  .collect()
        }
    }
}

impl From<Vec<String>> for RTG {
    // Convert a Vec<String> into an RTG.
    fn from(value: Vec<String>) -> Self {
        RTG {
            token_list: value
        }
    }
}

impl RandomTokenGenerator for RTG {
    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let tok = self.token_list.get(idx).unwrap_or(&String::from("")).clone();
        tok
    }
}


impl Display for RTG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTG({})", self.token_list.len())

    }
}