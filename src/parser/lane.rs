use bumpalo::Bump;
use winnow::{Parser, ascii::dec_int, combinator::{alt, opt, preceded, repeat}};

use crate::parser::{ErrorType, In, statement::{Id, Statement}, string::span_string_bump_parser, ws};

#[derive(Clone, Copy, Debug)]
pub enum Position<'arena>
{
    At(i64),
    Shares(Id<'arena>),
}

#[derive(Debug)]
pub struct Lane<'arena>
{
    id: Id<'arena>,
    display_name: &'arena str,
    position: Option<Position<'arena>>,
    style: Option<Id<'arena>>
}

impl<'arena> Position<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        alt((
            preceded("at", ws(dec_int)).map(Self::At),
            preceded("shares", ws(Statement::id_parser(bump))).map(Self::Shares)
        ))
    }
}

impl<'arena> Lane<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        preceded(
            "lane",
            ws((
                Statement::id_parser(bump),
                ws(span_string_bump_parser(bump)),
                ws(opt(Position::parser(bump))),
                opt(preceded("style", ws(Statement::id_parser(bump)))),
            ))
                .map(|(id, name, pos, style)| Self { id, display_name: name, position: pos, style}),
        )
}
}
