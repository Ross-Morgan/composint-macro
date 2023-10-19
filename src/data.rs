use quote::format_ident;
use syn::{token, Ident, braced, Token};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

use crate::sign::Sign;

use super::field::Field;

pub(crate) struct Data {
    _struct_token: Token![struct],
    pub(crate) type_name: Ident,
    _paren: token::Brace,
    pub(crate) fields: Punctuated<Field, Token![,]>,
}

impl Parse for Data {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(Data {
            _struct_token: input.parse()?,
            type_name: input.parse()?,
            _paren: braced!(content in input),
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

impl Data {
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }

    pub fn field_names(&self) -> Vec<Ident> {
        self
            .fields
            .pairs()
            .map(|p| p.value().name.clone())
            .collect()
    }

    pub fn field_sizes(&self) -> Vec<usize> {
        self
            .fields
            .pairs()
            .map(|p| p.value().bits.base10_parse().unwrap())
            .collect()
    }

    pub fn field_types(&self) -> Vec<Ident> {
        self
            .fields
            .pairs()
            .map(|p| {
                let size = p.value().bits.base10_parse().unwrap_or(32usize);
                let sign = p.value().sign;
                (size, sign)
            })
            .map(smallest_fitting_type)
            .collect()
    }

    pub fn field_cumulative_offsets(&self) -> Vec<usize> {
        self
            .fields
            .pairs()
            .map(|p| p.value().bits.base10_parse::<usize>().unwrap())
            .scan(0usize, |state, x| {
                *state += x;
                Some(*state - x)
            })
            .collect()
    }
}


fn smallest_fitting_type((bits, sign): (usize, Sign)) -> Ident {
    format_ident!("{}{}", sign.to_char(), bits.next_power_of_two())
}
