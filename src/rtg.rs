pub trait RandomTokenGenerator {
    fn new(token_list: Vec<impl ToString>) -> Self where Self: Sized;
    fn get_token(&self) -> String;
}
