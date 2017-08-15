use reader::lexer::Token;

use super::{Result, PullParser, State, DoctypeSubstate};

impl PullParser {
    pub fn inside_doctype(&mut self, t: Token, s: DoctypeSubstate) -> Option<Result> {
        match s {
            DoctypeSubstate::InsideName => {
                self.next_pos();
                self.lexer.disable_errors();
                self.into_state_continue(State::InsideDoctype(DoctypeSubstate::AfterName))
            },

            DoctypeSubstate::AfterName => match t {
                Token::TagEnd => {
                    self.lexer.enable_errors();
                    self.into_state_continue(State::OutsideTag)
                }

                _ => None
            }
        }
    }
}
