pub use ethabi::Token;
use ethereum_types::Address;

pub trait Tokenize {
    fn tokenize(&self) -> Token;
}

impl Tokenize for bool {
    fn tokenize(&self) -> Token {
        Token::Bool(*self)
    }
}

impl Tokenize for Address {
    fn tokenize(&self) -> Token {
        Token::Address(*self)
    }
}

impl<T: Tokenize> Tokenize for Vec<T> {
    fn tokenize(&self) -> Token {
        Token::Array(self.into_iter().map(Tokenize::tokenize).collect())
    }
}
