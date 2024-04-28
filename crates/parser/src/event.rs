use syntax::SyntaxKind;

use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Placeholder,
    Error(ParseError),
}
