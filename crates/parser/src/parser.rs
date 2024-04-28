pub mod marker;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use marker::Marker;
use syntax::SyntaxKind;

const RECOVERY_SET: [SyntaxKind; 1] = [SyntaxKind::LetKw];

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub fn bump(&mut self) {
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    pub fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn at_set(&mut self, set: &[SyntaxKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    pub fn error(&mut self) {
        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check;
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
