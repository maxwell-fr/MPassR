
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum SpecToken {
    LowercaseWord,
    UppercaseWord,
    PropercaseWord,
    RandomCapitalWord,
    LowercaseLetter,
    UppercaseLetter,
    AnyChar,
    AlphaNumChar,
    Digit,
    Symbol,
    Space,
    Shuffle,
}

impl SpecToken {

}

#[derive(Debug, Eq, PartialEq)]
pub enum SpecTokenError {
    UnrecognizedToken(char)
}

impl TryFrom<char> for SpecToken {
    type Error = SpecTokenError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(SpecToken::LowercaseWord),
            'W' => Ok(SpecToken::UppercaseWord),
            'i' => Ok(SpecToken::PropercaseWord),
            'r' => Ok(SpecToken::RandomCapitalWord),
            'a' => Ok(SpecToken::LowercaseLetter),
            'A' => Ok(SpecToken::UppercaseLetter),
            'x' => Ok(SpecToken::AlphaNumChar),
            'z' => Ok(SpecToken::AnyChar),
            '#' => Ok(SpecToken::Digit),
            '$' => Ok(SpecToken::Symbol),
            ' ' => Ok(SpecToken::Space),
            '?' => Ok(SpecToken::Shuffle),
            _ => return Err(SpecTokenError::UnrecognizedToken(value))
        }
    }
}