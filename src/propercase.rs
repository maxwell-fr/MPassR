use rand::{Rng, thread_rng};
use crate::rtg::RandomTokenGenerator;

pub struct PropercaseWordsRTG {
    token_list: Vec<String>,
}

impl PropercaseWordsRTG {
}

impl RandomTokenGenerator for PropercaseWordsRTG {
    fn new(token_list: Vec<impl ToString>) -> Self {
        PropercaseWordsRTG {
            token_list: token_list.iter()
                                  .map(|t| {
                                      let s = t.to_string();
                                      let mut c = s.chars();
                                      let first_char = c.next();
                                      if let Some(f) = first_char {
                                          format!("{}{}", f.to_ascii_uppercase(), c.collect::<String>())
                                      }
                                      else {
                                          String::new()
                                      }
                                  })
                                  .collect(),
        }
    }

    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let tok = self.token_list.get(idx).unwrap_or(&String::from("")).clone();
        tok
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtg() {
        let input_list = vec!["foo", "bar", "baz"];
        let output_list = vec!["Foo", "Bar", "Baz"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(PropercaseWordsRTG::new(input_list.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(output_list.contains(&t.as_str()));
        }
    }
}