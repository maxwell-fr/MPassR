//! Generate passphrases based on specification strings.

use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::rtg::{LowercaseWordsRTG, PropercaseWordsRTG, SymbolsRTG, UppercaseWordsRTG};
use crate::rtg::RandomTokenGenerator;
use crate::rtg::default_lists::{get_ez_ascii_symbols, get_simpleton_words};

pub struct Specifier {
    spec_tokens: Vec<Rc<dyn RandomTokenGenerator>>
}

impl Specifier {
    pub fn try_parse(spec_string: &str) -> Result<Self, ()> {
        Self::try_parse_custom(spec_string,
                               get_simpleton_words().iter().map(|s| s.to_string()).collect(),
                               get_ez_ascii_symbols().iter().map(|s| s.to_string()).collect())
    }

    pub fn try_parse_custom(spec_string: &str, word_list: Vec<String>, symbol_list: Vec<String>) -> Result<Self, ()> {
        if word_list.len() < 1 || symbol_list.len() < 1 {
            return Err(());
        }

        let lowercase = Rc::new(LowercaseWordsRTG::with_token_list(word_list.clone()));
        let uppercase = Rc::new(UppercaseWordsRTG::with_token_list(word_list.clone()));
        let propercase = Rc::new(PropercaseWordsRTG::with_token_list(word_list));
        let symbols = Rc::new(SymbolsRTG::with_token_list(symbol_list));
        let space = Rc::new(SymbolsRTG::with_token_list(vec![' ']));
        //todo: numbers

        let mut spec_tokens: Vec<Rc<dyn RandomTokenGenerator>> = Vec::new();

        for c in spec_string.chars() {
            let tok: Rc<dyn RandomTokenGenerator> = match c {
                'w' => lowercase.clone(),
                'u' => uppercase.clone(),
                'i' => propercase.clone(),
                //'r' => {}
                //'x' => {}
                //'z' => {}
                //'0' => {}
                '$' => symbols.clone(),
                ' ' => space.clone(),
                _ => return Err(())
            };
            spec_tokens.push(tok);
        }

        Ok(Specifier {
            spec_tokens
        })
    }

}

impl Display for Specifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for s in &self.spec_tokens {
            write!(f, "{} ", s)?
        }
        Ok(())
    }
}


//
// Specifier string syntax is simple.
// One or more of the following keys:
//     w - lowercase word (examnple)
//     u - uppercase word (EXAMPLE)
//     i - initial caps word (Example)
//     r - random cap word (exAmple)
//     x - random single alphanumeric character
//     z - random single alphanumeric or symbol character (combines x and $)
//     0 - digit, 0-9
//     $ - symbol
//     (space) - space character
// Examples:
//     "i w w 000$" => "Medium test phrase 123!"
//     "ii00$" => "TestPhrase11#"
//     "zzzzzz" => "t7Eq#r"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specifier() {
        let tester = Specifier::try_parse("w u i $").unwrap();


        println!("Specifier: {}", tester);
    }
}
