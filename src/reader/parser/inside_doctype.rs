use reader::lexer::Token;

use super::{Result, PullParser, State, DoctypeSubstate};

impl PullParser {
    pub fn inside_doctype(&mut self, t: Token, s: DoctypeSubstate) -> Option<Result> {
        match t {
            Token::TagEnd => {
                self.lexer.enable_errors();
                self.into_state_continue(State::OutsideTag)
            }

            _ => None
        }
    }
}
