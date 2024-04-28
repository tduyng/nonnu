mod expr;
mod stmt;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use syntax::SyntaxKind;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    while !p.at_end(){
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_multiple_statements() {
        check(
            "let a = 1\na",
            expect![[r#"
Root@0..11
  VariableDef@0..10
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Literal@8..10
      Number@8..9 "1"
      Whitespace@9..10 "\n"
  VariableRef@10..11
    Ident@10..11 "a""#]],
        );
    }

    #[test]
    fn recover_on_let_token() {
        check(
            "let a =\nlet b = a",
            expect![[r#"
Root@0..17
  VariableDef@0..8
    LetKw@0..3 "let"
    Whitespace@3..4 " "
    Ident@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 "\n"
  VariableDef@8..17
    LetKw@8..11 "let"
    Whitespace@11..12 " "
    Ident@12..13 "b"
    Whitespace@13..14 " "
    Equals@14..15 "="
    Whitespace@15..16 " "
    VariableRef@16..17
      Ident@16..17 "a""#]],
        );
    }
}