use reader::lexer::Token;

use super::{Result, PullParser, State, DoctypeSubstate, QualifiedNameTarget, SystemStartedSubstate, PublicStartedSubstate};

macro_rules! dispatch_on_enum_state(
    ($_self:ident, $s:expr, $c:expr, $is:expr,
     $($st:ident; $stc:expr ; $next_st:ident ; $chunk:expr),+;
     $end_st:ident ; $end_c:expr ; $end_chunk:expr ; $e:expr) => (
        match $s {
            $(
            $st => match $c {
                $stc  => $_self.into_state_continue(State::InsideDoctype($is($next_st))),
                c @ _ => Some(self_error!($_self; "Unexpected character '{}'", c))
            },
            )+
            $end_st => match $c {
                $end_c => $e,
                c @ _  => Some(self_error!($_self; "Unexpected character '{}'", c))
            }
        }
    )
);

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
                this.doctype_name = Some(name);
                this.into_state_continue(State::InsideDoctype(DoctypeSubstate::ExternalId))
            }),

            // We read the doctype name
            DoctypeSubstate::ExternalId => match t {
                Token::Whitespace(_) => None,

                Token::Character(c) if c == 'S' => self.into_state_continue(
                    State::InsideDoctype(DoctypeSubstate::SystemStarted(SystemStartedSubstate::S))),

                Token::Character(c) if c == 'P' => self.into_state_continue(
                    State::InsideDoctype(DoctypeSubstate::PublicStarted(PublicStartedSubstate::P))),

                _ => self.into_state_continue(State::InsideDoctype(DoctypeSubstate::InternalSubset))
            },

            DoctypeSubstate::SystemStarted(s) => self.system_started(t, s),
            DoctypeSubstate::PublicStarted(s) => self.public_started(t, s),

            DoctypeSubstate::SystemLiteral => self.read_system_literal(t, |this, value| {
                let system_literal = value;
                this.external_subset_uri = Some(system_literal);
                this.into_state_continue(State::InsideDoctype(DoctypeSubstate::InternalSubset))
            }),

            DoctypeSubstate::PubidLiteral => match t {
                Token::Whitespace(_) => None,

                _ => {
                    self.lexer.disable_errors();
                    self.into_state_continue(State::InsideDoctype(DoctypeSubstate::InternalSubset))
                }
            },

            DoctypeSubstate::InternalSubset => match t {
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

    pub fn system_started(&mut self, t: Token, s: SystemStartedSubstate) -> Option<Result> {
        use self::SystemStartedSubstate::*;

        if let Token::Character(c) = t {
            dispatch_on_enum_state!(self, s, c, DoctypeSubstate::SystemStarted,
                                    S     ; 'Y' ; SY    ; "S",
                                    SY    ; 'S' ; SYS   ; "SY",
                                    SYS   ; 'T' ; SYST  ; "SYS",
                                    SYST  ; 'E' ; SYSTE ; "SYST";
                                    SYSTE ; 'M' ; "SYSTE" ; self.into_state_continue(State::InsideDoctype(DoctypeSubstate::SystemLiteral))
            )
        } else {
            Some(self_error!(self; "Unexpected token '{}'", t))
        }
    }

    pub fn public_started(&mut self, t: Token, s: PublicStartedSubstate) -> Option<Result> {
        use self::PublicStartedSubstate::*;

        if let Token::Character(c) = t {
            dispatch_on_enum_state!(self, s, c, DoctypeSubstate::PublicStarted,
                                    P     ; 'U' ; PU    ; "P",
                                    PU    ; 'B' ; PUB   ; "PU",
                                    PUB   ; 'L' ; PUBL  ; "PUB",
                                    PUBL  ; 'I' ; PUBLI ; "PUBL";
                                    PUBLI ; 'C' ; "PUBLI" ; self.into_state_continue(State::InsideDoctype(DoctypeSubstate::PubidLiteral))
            )
        } else {
            Some(self_error!(self; "Unexpected token '{}'", t))
        }
    }
}
