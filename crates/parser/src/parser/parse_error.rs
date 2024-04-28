use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub expected: Vec<SyntaxKind>,
    pub found: Option<SyntaxKind>,
    pub range: TextRange,
}
