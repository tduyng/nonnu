use crate::lexer::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    StartNode {
        kind: TokenKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
}
