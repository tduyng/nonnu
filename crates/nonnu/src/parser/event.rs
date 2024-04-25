use crate::lexer::SyntaxKind;

#[derive(Debug, Clone)]
pub enum Event {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: String },
    FinishNode,
}
