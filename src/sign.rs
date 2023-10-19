use syn::{parse::{Parse, ParseStream}, Ident, Result};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Sign {
    #[default]
    Signed,
    Unsigned,
}

impl Parse for Sign {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;

        let s = match ident.to_string().to_lowercase().as_str() {
            "i" | "s" | "signed" => Self::Signed,
            "u" | "unsigned" => Self::Unsigned,
            _ => panic!("Invalid sign identifier")
        };

        Ok(s)
    }
}

impl Sign {
    pub(crate) const fn to_char(self) -> char {
        match self {
            Self::Signed => 'i',
            Self::Unsigned => 'u',
        }
    }
}
