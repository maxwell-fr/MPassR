use rand::{Rng, thread_rng};

pub trait RandomTokenGenerator {
    fn new(token_list: Vec<impl ToString>) -> Self where Self: Sized;
    fn with_transformer<T, F>(token_list: Vec<T>, transformer: F) -> Self
        where Self: Sized,
        T: ToString,
        F: Fn(String) -> String + 'static;
    fn get_token(&self) -> String;
}

struct Tester1 {
    token_list: Vec<String>,
    transformer: Box<dyn Fn(String) -> String>
}

impl Tester1 {
}

impl RandomTokenGenerator for Tester1 {
    fn new(token_list: Vec<impl ToString>) -> Self {
        Self::with_transformer(token_list, Box::new(|s| s))
    }

    fn with_transformer<T, F>(token_list: Vec<T>, transformer: F) -> Self
          where T: ToString,
                F: Fn(String) -> String + 'static
    {
        Tester1 {
            token_list: token_list.iter()
                                  .map(|t| t.to_string())
                                  .collect(),
            transformer: Box::new(transformer)
        }
    }

    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.token_list.len());
        let tok = self.token_list.get(idx).unwrap_or(&String::from("")).clone();
        format!("{}", (self.transformer)(tok))
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_generator() {
        let v = vec!["foo", "bar", "baz"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(Tester1::new(v.clone()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(v.contains(&t.as_str()));
        }
    }

    #[test]
    fn transforming_token_generator() {
        let v = vec!["foo", "bar", "baz"];
        let o = vec!["FOO", "BAR", "BAZ"];
        let tester = Box::new(Tester1::with_transformer(v.clone(),
                                                        |s| s.to_ascii_uppercase()));

        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(o.contains(&t.as_str()));
        }
    }
}
