//! Generate passphrases based on specification strings.

use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::rtg::RTG;
use crate::rtg::RandomTokenGenerator;
use crate::rtg::default_lists::{get_alphabet, get_ez_ascii_symbols, get_numbers, get_simpleton_words};

pub struct Specifier {
    spec_tokens: Vec<Rc<dyn RandomTokenGenerator>>,
    rtgs: RTGenerators
}

struct RTGenerators {
    lowercase: Rc<RTG>,
    uppercase: Rc<RTG>,
    propercase: Rc<RTG>,
    symbols: Rc<RTG>,
    space: Rc<RTG>,
    numbers: Rc<RTG>,
    alphabet: Rc<RTG>,
    alphanum: Rc<RTG>,
    alphanumsym: Rc<RTG>
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
    pub fn try_parse(spec_string: &str) -> Result<Self, ()> {
        Self::try_parse_custom(spec_string,
                               get_simpleton_words().iter().map(|s| s.to_string()).collect(),
                               get_ez_ascii_symbols().iter().map(|s| s.to_string()).collect())
    }

    /// Try to parse a spec string and build a Specifier using custom word
    /// and symbol lists.
    /// Returns a Result containing a new Specifier or an Error with failure details.
    pub fn try_parse_custom(spec_string: &str, word_list: Vec<String>, symbol_list: Vec<String>) -> Result<Self, ()> {
        if word_list.len() < 1 || symbol_list.len() < 1 {
            return Err(());
        }

        let lowercase = Rc::new(RTG::from(word_list.clone().iter().map(|s| s.to_string().to_ascii_lowercase()).collect::<Vec<String>>()));
        let uppercase = Rc::new(RTG::from(word_list.clone().iter().map(|s| s.to_string().to_ascii_uppercase()).collect::<Vec<String>>()));
        let propercase = Rc::new(RTG::from(word_list.clone().iter().map(|t| {
            let s = t.to_string();
            let mut c = s.chars();
            let first_char = c.next();
            if let Some(f) = first_char {
                format!("{}{}", f.to_ascii_uppercase(), c.collect::<String>())
            }
            else {
                String::new()
            }
        }).collect::<Vec<String>>()));
        let symbols = Rc::new(RTG::from(symbol_list.clone()));
        let space = Rc::new(RTG::new(vec![' ']));
        let numbers = Rc::new(RTG::new(get_numbers()));
        let alphabet = Rc::new(RTG::new(get_alphabet()));
        let mut alphanum: Vec<String> = get_alphabet().iter().map(|s| s.to_string()).collect();
        alphanum.extend(get_numbers().iter().map(|s| s.to_string()));
        let mut alphanumsym = alphanum.clone();
        alphanumsym.extend(symbol_list.clone());
        let alphanum = Rc::new(RTG::new(alphanum));
        let alphanumsym = Rc::new(RTG::new(alphanumsym));

        let rtgs = RTGenerators {
            lowercase,
            uppercase,
            propercase,
            symbols,
            space,
            numbers,
            alphabet,
            alphanum,
            alphanumsym
        };

        let spec_tokens = Self::tokenize(spec_string, &rtgs);

        if spec_tokens.is_err() {
            Err(())
        }
        else {
            Ok(Specifier {
                spec_tokens: spec_tokens.unwrap(),
                rtgs
            })
        }
    }

    /// Validate a spec string.
    /// Returns a Result with either Ok or an Err containing the offset of the first
    /// invalid character.
    pub fn check_spec_string(spec_string: &str) -> Result<(), usize> {
        let allowed = "wuirxz0$ ";
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
    pub fn try_change_spec_string(&mut self, spec_string: &str) -> Result<(), usize>{
        let spec_tokens = Self::tokenize(spec_string, &self.rtgs);

        match spec_tokens {
            Err(e) => {
                Err(e)
            }
            Ok(tokens) => {
                self.spec_tokens = tokens;
                Ok(())
            }
        }
    }


    fn tokenize(spec_string: &str, rtgs: &RTGenerators) -> Result<Vec<Rc<dyn RandomTokenGenerator>>, usize> {
        let mut spec_tokens: Vec<Rc<dyn RandomTokenGenerator>> = Vec::new();

        for (index, ch) in spec_string.char_indices() {
            let tok: Rc<dyn RandomTokenGenerator> = match ch {
                'w' => rtgs.lowercase.clone(),
                'u' => rtgs.uppercase.clone(),
                'i' => rtgs.propercase.clone(),
                //'r' => {}
                'a' => rtgs.alphabet.clone(),
                'x' => rtgs.alphanum.clone(),
                'z' => rtgs.alphanumsym.clone(),
                '0' => rtgs.numbers.clone(),
                '$' => rtgs.symbols.clone(),
                ' ' => rtgs.space.clone(),
                _ => return Err(index)
            };
            spec_tokens.push(tok);
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
        let tester = Specifier::try_parse("w x u i $ 0 x z").unwrap();

        println!("Specifier: {}", tester);
    }

    #[test]
    fn test_passphrase() {
        let tester = Specifier::try_parse("w x u i $ 0 x z").unwrap();

        println!("Passphrase: {}", tester.get_passphrase());
    }
}
