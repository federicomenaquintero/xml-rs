use reader::lexer::Token;

use super::{Result, PullParser, State, DoctypeSubstate, QualifiedNameTarget};

impl PullParser {
    pub fn inside_doctype(&mut self, t: Token, s: DoctypeSubstate) -> Option<Result> {
        macro_rules! unexpected_token(($t:expr) => (Some(self_error!(self; "Unexpected token inside doctype declaration: {}", $t))));

        match s {
            // We read "<!DOCTYPE"; whitespace comes next
            DoctypeSubstate::AfterDoctype => match t {
                Token::Whitespace(_) => self.into_state_continue(State::InsideDoctype(DoctypeSubstate::InsideName)),
                _ => unexpected_token!(t)
            },

            // Doctype name
            DoctypeSubstate::InsideName => self.read_qualified_name(t, QualifiedNameTarget::DoctypeNameTarget, |this, token, name| {
                // FIXME: do something with the name?
                this.into_state_continue(State::InsideDoctype(DoctypeSubstate::AfterName))
            }),

            // We read the doctype name
            DoctypeSubstate::AfterName => match t {
                Token::Whitespace(_) => None,

                Token::TagEnd => {
                    self.lexer.enable_errors();
                    self.into_state_continue(State::OutsideTag)
                },

                _ => {
                    self.lexer.disable_errors();
                    None
                }
            }
        }
    }
}
