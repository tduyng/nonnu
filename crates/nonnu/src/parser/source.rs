use crate::lexer::{SyntaxKind, Token};

pub struct Source<'l, 'input> {
    pub lexemes: &'l [Token<'input>],
    pub cursor: usize,
}

impl<'l, 'input> Source<'l, 'input> {
    pub fn new(lexemes: &'l [Token<'input>]) -> Self {
        Self { lexemes, cursor: 0 }
    }

    pub fn next_lexeme(&mut self) -> Option<&'l Token<'input>> {
        self.eat_trivia();

        let lexeme = self.lexemes.get(self.cursor)?;
        self.cursor += 1;

        Some(lexeme)
    }

    pub fn peek_kind(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, SyntaxKind::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<SyntaxKind> {
        self.lexemes.get(self.cursor).map(|Token { kind, .. }| *kind)
    }
}
