//! [`RandomTokenGenerator`] trait
pub mod lowercase;
pub mod uppercase;
pub mod propercase;
pub mod symbol;
pub mod number;
pub mod alphabet;
pub mod default_lists;

pub use lowercase::LowercaseWordsRTG;
pub use uppercase::UppercaseWordsRTG;
pub use propercase::PropercaseWordsRTG;
pub use symbol::SymbolsRTG;
pub use number::NumbersRTG;

use std::fmt::Display;

/// The shared trait for all token generators.
pub trait RandomTokenGenerator : Display {
    /// Get a new RandomTokenGenerator
    /// This uses a default list of tokens provided by the trait implementor.
    fn new() -> Self where Self: Sized;

    /// Get a new RandomTokenGenerator that will use provided tokens.
    /// Accepts a vector of anything that can be converted to String.
    fn with_token_list(token_list: Vec<impl ToString>) -> Self where Self: Sized;

    /// Gets a token. The rules for the provided token are
    /// dependent on the trait implementor.
    fn get_token(&self) -> String;
}
