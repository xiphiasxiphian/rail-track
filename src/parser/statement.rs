use bumpalo::Bump;
use winnow::{
    Parser, ascii::{alpha1, alphanumeric1, dec_int, dec_uint}, combinator::{alt, dispatch, empty, fail, opt, preceded, repeat, separated_pair, todo}, token::one_of,
};

use crate::parser::{ErrorType, In, ParseResult, kw_slice, lane::{Lane, LaneSelection}, string::span_string_bump_parser, ws};

#[derive(Clone, Copy, Debug)]
pub struct Id<'arena>(&'arena str);

#[derive(Clone, Copy, Debug)]
pub struct Distance(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor
{
    Start,
    End,
}

#[derive(Clone, Copy, Debug)]
pub enum Location<'arena>
{
    Distance(Distance),
    Mark(Id<'arena>),
    Offset(Id<'arena>, i64),
    Anchor(Id<'arena>, Anchor)
}

pub enum Statement<'arena>
{
    Axis(/* horizontal / vertical */),
    Scale(/* schematic / true */),
    Style
    {
        id: Id<'arena>,

    },
    Lane(Lane<'arena>),
    Mark
    {
        id: Id<'arena>,
        distance: Distance,
    },
    Run
    {
        lanes: LaneSelection<'arena>,
        from: Location<'arena>,
        to: Location<'arena>,
    },
}

impl<'arena> Statement<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        alt((
            Lane::parser(bump).map(Self::Lane),
            Self::mark_parser(bump))
        )
    }

    pub fn id_parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Id<'arena>, ErrorType>
    {
        (
            alt((alpha1, "_")),
            repeat(0.., alt((alphanumeric1, "_"))).fold(|| 0, |acc, _| acc + 1),
        )
            .take()
            .map(|x| Id(bump.alloc_str(x)))
    }

    fn mark_parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        preceded(
            "mark",
            ws(separated_pair(Self::id_parser(bump), ws('='), dec_uint)).map(|(id, distance)| Self::Mark {
                id,
                distance: Distance(distance),
            }),
        )
    }
}

#[derive(Clone, Copy, Debug)]
enum Suffix
{
    Anchor(Anchor),
    Offset(i64)
}

impl<'arena> Location<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        alt((
            dec_uint.map(|n: u64| Self::Distance(Distance(n))),
            (
                Statement::id_parser(bump),
                opt(alt((
                    preceded('.', Self::parse_anchor).map(Suffix::Anchor),
                    dec_int.map(Suffix::Offset)
                )))
            )
            .map(|(id, suffix)| match suffix {
                None => Self::Mark(id),
                Some(Suffix::Anchor(anchor)) => Self::Anchor(id, anchor),
                Some(Suffix::Offset(offset)) => Self::Offset(id, offset),
            })
        ))

        // TODO: error stuff idk
    }

    fn parse_anchor(input: &mut In) -> ParseResult<Anchor>
    {
        dispatch! {
            kw_slice;
            "start" => empty.value(Anchor::Start),
            "end" => empty.value(Anchor::End),
            _ => fail
        }.parse_next(input)
    }
}
