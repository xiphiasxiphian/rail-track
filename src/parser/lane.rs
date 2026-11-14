use bumpalo::Bump;
use winnow::{Parser, ascii::{dec_int, float}, combinator::{alt, delimited, opt, preceded, repeat}};

use crate::parser::{ErrorType, In, closing_brace, kw, statement::{Id, Statement}, string::span_string_bump_parser, ws};

#[derive(Clone, Copy, Debug)]
pub enum Position<'arena>
{
    At(f64),
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

#[derive(Clone, Copy, Debug)]
pub enum GroupBody<'arena>
{
    Lanes(&'arena [Lane<'arena>]),
    Alias(),
}

#[derive(Clone, Copy, Debug)]
pub struct Group<'arena>
{
    id: Id<'arena>,
    body: GroupBody<'arena>,
}

impl<'arena> Position<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        alt((
            preceded("at", ws(float)).map(Self::At),
            preceded("shares", ws(Statement::id_parser(bump))).map(Self::Shares)
        ))
    }
}

impl<'arena> Lane<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        preceded(
            kw("lane"),
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

impl<'arena> Group<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        preceded(
            kw("group"),
            (
                ws(Statement::id_parser(bump)),
                alt((
                    delimited(ws('{'), closing_brace())
                ))
            )
        )
    }
}
