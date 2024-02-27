use rand::{Rng, thread_rng};

pub trait RandomTokenGenerator {
    fn new(token_list: Vec<impl ToString>) -> Self where Self: Sized;
    fn get_token(&self) -> String;
}

struct Tester1 {
    l: Vec<String>
}

impl Tester1 {
}

impl RandomTokenGenerator for Tester1 {
    fn new(token_list: Vec<impl ToString>) -> Tester1 {
        Tester1 {
            l: token_list.iter()
                         .map(|t| t.to_string())
                         .collect()
        }
    }

    fn get_token(&self) -> String {
        let idx: usize = thread_rng().gen_range(0 .. self.l.len());
        format!("{}", self.l[idx])
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_generator() {
        let v = vec!["1", "2", "3"];
        let tester: Box<dyn RandomTokenGenerator> = Box::new(Tester1::new(v.clone()));

        println!("Token: {}", tester.get_token());
        println!("Token: {}", tester.get_token());
        println!("Token: {}", tester.get_token());
        println!("Token: {}", tester.get_token());
        println!("Token: {}", tester.get_token());
        for _ in 0..100 {
            let t = tester.get_token();
            assert!(v.contains(&t.as_str()));
        }
    }
}
