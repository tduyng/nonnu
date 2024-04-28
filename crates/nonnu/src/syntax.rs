use crate::lexer::TokenKind;
use num_traits::{FromPrimitive, ToPrimitive};
pub type SyntaxNode = rowan::SyntaxNode<NonnuLanguage>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NonnuLanguage {}

impl rowan::Language for NonnuLanguage {
    type Kind = TokenKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}
