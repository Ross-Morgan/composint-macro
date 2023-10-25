use syn::{Ident, LitInt, Token};
use syn::parse::{Parse, ParseStream, Result};

pub(crate) struct Field {
    pub(crate) name: Ident,
    _colon: Token![:],
    pub(crate) bits: LitInt,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            _colon: input.parse()?,
            bits: input.parse()?,
        })
    }
}
