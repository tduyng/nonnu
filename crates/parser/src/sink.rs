use crate::{parser::ParseError, Parse};

use super::event::Event;
use lexer::Token;
use rowan::{GreenNodeBuilder, Language};
use std::mem;
use syntax::NonnuLanguage;

pub struct Sink<'t, 'input> {
    pub builder: GreenNodeBuilder<'static>,
    pub tokens: &'t [Token<'input>],
    pub cursor: usize,
    pub events: Vec<Event>,
    pub errors: Vec<ParseError>,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::new(),
        }
    }

    pub fn finish(mut self) -> Parse {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode { kind, forward_parent } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;
                    let mut forward_parent = forward_parent;

                    while let Some(fp) = forward_parent {
                        idx += fp;

                        forward_parent = if let Event::StartNode { kind, forward_parent } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(NonnuLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
                Event::Error(error) => self.errors.push(error),
            }

            self.eat_trivia();
        }

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if !token.kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    fn token(&mut self) {
        let Token { kind, text, .. } = self.tokens[self.cursor];

        self.builder.token(NonnuLanguage::kind_to_raw(kind.into()), text);

        self.cursor += 1;
    }
}
