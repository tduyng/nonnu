mod token_kind;

pub use token_kind::TokenKind;

use std::convert::TryFrom;
use std::ops::Range as StdRange;
use text_size::{TextRange, TextSize};

use logos::Logos;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        match kind {
            Ok(kind) => {
                let text = self.inner.slice();

                let range = {
                    let StdRange { start, end }: std::ops::Range<usize> = self.inner.span();
                    let start = TextSize::try_from(start).unwrap();
                    let end = TextSize::try_from(end).unwrap();

                    TextRange::new(start, end)
                };

                Some(Self::Item { kind, text, range })
            }
            Err(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}
