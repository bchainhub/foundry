//! Formatting helpers for [`Token`]s.

use corebc_core::{abi::Token, types::I256, utils::hex};
use std::{fmt, fmt::Write};

/// Wrapper that pretty formats a token
pub struct TokenDisplay<'a>(pub &'a Token);

impl fmt::Display for TokenDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_token(f, self.0)
    }
}

/// Recursively formats an ABI token.
fn fmt_token(f: &mut fmt::Formatter, item: &Token) -> fmt::Result {
    match item {
        Token::Address(inner) => {
            write!(f, "{}", inner)
        }
        // add 0x
        Token::Bytes(inner) => write!(f, "0x{}", hex::encode(inner)),
        Token::FixedBytes(inner) => write!(f, "0x{}", hex::encode(inner)),
        // print as decimal
        Token::Uint(inner) => write!(f, "{inner}"),
        Token::Int(inner) => write!(f, "{}", I256::from_raw(*inner)),
        Token::Array(tokens) | Token::FixedArray(tokens) => {
            f.write_char('[')?;
            let mut tokens = tokens.iter().peekable();
            while let Some(token) = tokens.next() {
                fmt_token(f, token)?;
                if tokens.peek().is_some() {
                    f.write_char(',')?
                }
            }
            f.write_char(']')
        }
        Token::Tuple(tokens) => {
            f.write_char('(')?;
            let mut tokens = tokens.iter().peekable();
            while let Some(token) = tokens.next() {
                fmt_token(f, token)?;
                if tokens.peek().is_some() {
                    f.write_char(',')?
                }
            }
            f.write_char(')')
        }
        _ => write!(f, "{item}"),
    }
}
