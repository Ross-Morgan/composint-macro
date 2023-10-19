use syn::{Ident, LitInt, Token};
use syn::parse::{Parse, ParseStream, Result};

use super::sign::Sign;

pub(crate) struct Field {
    pub(crate) name: Ident,
    _colon: Token![:],
    pub(crate) sign: Sign,
    pub(crate) bits: LitInt,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            _colon: input.parse()?,
            sign: input.parse()?,
            bits: input.parse()?,
        })
    }
}
