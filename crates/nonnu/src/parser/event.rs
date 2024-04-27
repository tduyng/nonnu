use crate::lexer::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        text: String,
    },
    FinishNode,
    Placeholder,
}
