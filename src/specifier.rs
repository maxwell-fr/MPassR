//! Generate passphrases based on specification strings.

mod spectoken;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::rtg::{RandomCapWordGenerator, RTG};
use crate::rtg::RandomTokenGenerator;
use crate::rtg::default_lists::{get_alphabet, get_ez_ascii_symbols, get_lowercase, get_numbers, get_simpleton_words, get_uppercase};
use crate::specifier::spectoken::{SpecToken, SpecTokenError};

pub struct Specifier {
    spec_tokens: Vec<Rc<dyn RandomTokenGenerator>>,
    rtgs: HashMap<SpecToken, Rc<dyn RandomTokenGenerator>>
}

#[derive(Debug, Eq, PartialEq)]
pub enum SpecifierError {
    SpecTokenError(SpecTokenError),
    EmptySymbolList,
    EmptyWordList,
    UnrecognizedChar(usize)
}


impl From<SpecTokenError> for SpecifierError {
    fn from(value: SpecTokenError) -> Self {
        SpecifierError::SpecTokenError(value)
    }
}


impl Specifier {
    /// Use the rules encoded in the spec string to produce a passphrase.
    pub fn get_passphrase(&self) -> String {
        let mut p = String::new();

        for r in self.spec_tokens.iter() {
            p.push_str(&r.get_token());
        }

        p
    }

    /// Try to parse a spec string a build a Specifier using the default word
    /// and symbol lists.
    /// Returns a Result containing a new Specifier or an Error with failure details.
    pub fn try_parse(spec_string: &str) -> Result<Self, SpecifierError> {
        Self::try_parse_custom(spec_string,
                               get_simpleton_words().iter().map(|s| s.to_string()).collect(),
                               get_ez_ascii_symbols().iter().map(|s| s.to_string()).collect())
    }

    /// Try to parse a spec string and build a Specifier using custom word
    /// and symbol lists.
    /// Returns a Result containing a new Specifier or an Error with failure details.
    pub fn try_parse_custom(spec_string: &str, word_list: Vec<String>, symbol_list: Vec<String>) -> Result<Self, SpecifierError> {
        if word_list.is_empty() || symbol_list.is_empty() {
            return Err(SpecifierError::EmptyWordList);
        }
        if symbol_list.is_empty() {
            return Err(SpecifierError::EmptySymbolList);
        }

        let mut rtgs: HashMap<SpecToken, Rc<dyn RandomTokenGenerator>> = HashMap::new();
        rtgs.insert(SpecToken::LowercaseWord,
                    Rc::new(RTG::from(word_list.clone().iter()
                                                              .map(|s| s.to_string().to_ascii_lowercase())
                                                              .collect::<Vec<String>>())));
        rtgs.insert(SpecToken::UppercaseWord,
                    Rc::new(RTG::from(word_list.clone().iter()
                                                              .map(|s| s.to_string().to_ascii_uppercase())
                                                              .collect::<Vec<String>>())));
        rtgs.insert(SpecToken::PropercaseWord,
                    Rc::new(RTG::from(word_list.clone().iter()
                                                              .map(|t| {
                                                                  let s = t.to_string();
                                                                  let mut c = s.chars();
                                                                  let first_char = c.next();

                                                                  if let Some(f) = first_char {
                                                                      format!("{}{}", f.to_ascii_uppercase(), c.collect::<String>())
                                                                  } else {
                                                                      String::new()
                                                                  }
                                                              })
                                                              .collect::<Vec<String>>())));
        rtgs.insert(SpecToken::Symbol,
                    Rc::new(RTG::from(symbol_list.clone())));
        rtgs.insert(SpecToken::Space,
                    Rc::new(RTG::new(vec![' '])));
        rtgs.insert(SpecToken::RandomCapitalWord,
                    Rc::new(RandomCapWordGenerator::from(word_list.clone().iter()
                                   .map(|s| s.to_string().to_ascii_lowercase())
                                   .collect::<Vec<String>>())));
        rtgs.insert(SpecToken::Digit,
                    Rc::new(RTG::new(get_numbers())));
        rtgs.insert(SpecToken::LowercaseLetter,
                    Rc::new(RTG::new(get_lowercase())));
        rtgs.insert(SpecToken::UppercaseLetter,
                    Rc::new(RTG::new(get_uppercase())));
        let mut alphanum: Vec<String> = get_alphabet().iter().map(|s| s.to_string()).collect();
        alphanum.extend(get_numbers().iter().map(|s| s.to_string()));
        let mut alphanumsym = alphanum.clone();
        alphanumsym.extend(symbol_list.clone());
        rtgs.insert(SpecToken::AlphaNumChar,
                    Rc::new(RTG::new(alphanum)));
        rtgs.insert(SpecToken::AnyChar,
                    Rc::new(RTG::new(alphanumsym)));


        let spec_tokens = Self::tokenize(spec_string, &rtgs)?;

        Ok(Specifier {
            spec_tokens,
            rtgs
        })
    }

    /// Validate a spec string.
    /// Returns a Result with either Ok or an Err containing the offset of the first
    /// invalid character.
    pub fn check_spec_string(spec_string: &str) -> Result<(), usize> {
        let allowed = "wWaAirxz#$ ";
        for (index, ch) in spec_string.char_indices() {
           if !allowed.contains(ch) {
               return Err(index)
           }
        }

        Ok(())
    }

    /// Try to change the spec string to a new one without reloading word lists.
    /// Returns a Result with either Ok or an Err containing the offset of the first
    /// invalid character. The original spec string is not changed on failure.
    pub fn try_change_spec_string(&mut self, spec_string: &str) -> Result<(), SpecifierError>{
        self.spec_tokens = Self::tokenize(spec_string, &self.rtgs)?;
        Ok(())
    }


    fn tokenize(spec_string: &str, rtgs: &HashMap<SpecToken, Rc<dyn RandomTokenGenerator>>) -> Result<Vec<Rc<dyn RandomTokenGenerator>>, SpecifierError> {
        let mut spec_tokens: Vec<Rc<dyn RandomTokenGenerator>> = Vec::new();

        for (index, ch) in spec_string.char_indices() {
            if let Ok(tok) = SpecToken::try_from(ch){
                spec_tokens.push(rtgs[&tok].clone());
            }
            else {
                return Err(SpecifierError::UnrecognizedChar(index))
            }
        }

        Ok(spec_tokens)
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
//     W - uppercase word (EXAMPLE)
//     i - initial caps word (Example)
//     r - random cap word (exAmple)
//     a - random lowercase letter
//     A - random uppercase letter
//     x - random single alphanumeric character
//     z - random single alphanumeric or symbol character (combines x and $)
//     # - digit, 0-9
//     $ - symbol
//     (space) - space character
// Examples:
//     "i w w ###$" => "Medium test phrase 123!"
//     "ii##$" => "TestPhrase11#"
//     "zzzzzz" => "t7Eq#r"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specifier() {
        let tester = Specifier::try_parse("w W i r a A $ # x z").unwrap();

        println!("Specifier: {}", tester);
    }

    #[test]
    fn test_passphrase() {
        let tester = Specifier::try_parse("w W i r a A $ # x z").unwrap();

        println!("Passphrase: {}", tester.get_passphrase());
    }
}
