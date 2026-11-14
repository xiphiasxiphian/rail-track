use bumpalo::Bump;
use winnow::{Parser, ascii::{dec_int, float}, combinator::{alt, delimited, opt, preceded, repeat, separated}};

use crate::parser::{ErrorType, In, block, closing_brace, kw, statement::{Id, Statement}, string::span_string_bump_parser, ws};

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
    Alias(LaneSelection<'arena>),
}

#[derive(Clone, Copy, Debug)]
pub struct Group<'arena>
{
    id: Id<'arena>,
    body: GroupBody<'arena>,
}

#[derive(Clone, Copy, Debug)]
pub enum LaneSelection<'arena>
{
    Single(Id<'arena>),
    List(&'arena [LaneSelection<'arena>]),
    Range(Id<'arena>, Id<'arena>),
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
                    block(repeat(0.., ws(Lane::parser(bump))))
                        .map(|lanes: Vec<Lane<'arena>>| GroupBody::Lanes(bump.alloc_slice_fill_iter(lanes))),
                    preceded(ws('='), LaneSelection::parser(bump)).map(GroupBody::Alias),
                ))
            )
            .map(|(id, body)| Self { id, body })
        )
    }
}

impl<'arena> LaneSelection<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        alt((
            delimited(ws('['), Self::items_parser(bump), ws(']'))
                .map(|items| Self::List(bump.alloc_slice_copy(&items))),
        ))
    }

    pub fn item_parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        (
            Statement::id_parser(bump),
            opt(preceded("..", Statement::id_parser(bump))),
        )
        .map(|(from, to)| match to {
            Some(to) => Self::Range(from, to),
            None => Self::Single(from)
        })
    }

    pub fn items_parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Vec<Self>, ErrorType>
    {
        separated(1.., Self::item_parser(bump), ws(','))
    }
}
