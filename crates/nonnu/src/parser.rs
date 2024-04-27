mod event;
mod expr;
mod marker;
mod sink;
mod source;

pub use event::*;
pub use expr::*;
pub use marker::*;
pub use sink::*;
pub use source::*;

use crate::{
    lexer::{Lexer, SyntaxKind, Token},
    syntax::SyntaxNode,
};
use rowan::GreenNode;

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let token_refs: Vec<Token<'_>> = tokens.iter().map(|&(kind, text)| Token { kind, text }).collect();

    let parser = Parser::new(&token_refs);
    let events = parser.parse();
    let sink = Sink::new(&token_refs, events);

    Parse {
        green_node: sink.finish(),
    }
}

pub struct Parser<'l, 'input> {
    pub source: Source<'l, 'input>,
    pub events: Vec<Event>,
}

impl<'l, 'input> Parser<'l, 'input> {
    fn new(tokens: &'l [Token<'input>]) -> Self {
        Self {
            source: Source::new(tokens),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        let m = self.start();
        expr(&mut self);
        m.complete(&mut self, SyntaxKind::Root);

        self.events
    }

    fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }
}

pub struct Parse {
    pub green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        );
    }
}
