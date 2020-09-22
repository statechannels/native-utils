use ethabi::encode;

use super::tokenize::*;

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl<T: Tokenize> Encode for T {
    fn encode(&self) -> Vec<u8> {
        encode(&[self.tokenize()])
    }
}
