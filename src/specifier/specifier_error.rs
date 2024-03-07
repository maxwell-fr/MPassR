use crate::specifier::spectoken::SpecTokenError;

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
