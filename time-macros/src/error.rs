use std::fmt;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::format_description::error::InvalidFormatDescription;

pub(crate) enum Error {
    MissingComponent { name: &'static str },
    InvalidComponent { name: &'static str, value: String },
    ExpectedString,
    UnexpectedToken { tree: TokenTree },
    UnexpectedEndOfInput,
    InvalidFormatDescription(InvalidFormatDescription),
    Custom(String),
}

impl From<InvalidFormatDescription> for Error {
    fn from(v: InvalidFormatDescription) -> Self {
        Self::InvalidFormatDescription(v)
    }
}

impl fmt::Display for Error {
    #[allow(clippy::use_self)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingComponent { name } => write!(f, "missing component: {}", name),
            Self::InvalidComponent { name, value } => {
                write!(f, "invalid component: {} was {}", name, value)
            }
            Self::ExpectedString => f.write_str("expected string"),
            Self::UnexpectedToken { tree } => write!(f, "unexpected token: {}", tree),
            Self::UnexpectedEndOfInput => f.write_str("unexpected end of input"),
            Self::InvalidFormatDescription(err) => err.fmt(f),
            Self::Custom(s) => f.write_str(s),
        }
    }
}

impl Error {
    pub(crate) fn to_compile_error(&self) -> TokenStream {
        [
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
            TokenStream::from(TokenTree::Ident(Ident::new("core", Span::call_site()))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Joint))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
            TokenStream::from(TokenTree::Ident(Ident::new(
                "compile_error",
                Span::call_site(),
            ))),
            TokenStream::from(TokenTree::Punct(Punct::new('!', Spacing::Alone))),
            TokenStream::from(TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                TokenStream::from(TokenTree::Literal(Literal::string(&self.to_string()))),
            ))),
        ]
        .iter()
        .cloned()
        .collect()
    }
}
