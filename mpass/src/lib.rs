//! A library for generating random passphrases.
//!
//! MPass provides a means to generate passphrases based on customizable
//! word lists, numbers, symbols, and other tokens.


pub mod rtg;
pub mod specifier;

pub use specifier::Specifier;
pub use rtg::RandomTokenGenerator;



