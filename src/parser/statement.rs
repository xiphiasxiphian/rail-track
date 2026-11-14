use bumpalo::Bump;
use winnow::{
    Parser,
    ascii::{alpha1, alphanumeric1, dec_uint},
    combinator::{alt, preceded, repeat, separated_pair},
};

use crate::parser::{ErrorType, In, lane::Lane, string::span_string_bump_parser, ws};

#[derive(Clone, Copy, Debug)]
pub struct Id<'arena>(&'arena str);

#[derive(Clone, Copy, Debug)]
pub struct Distance(u64);

#[derive(Clone, Copy, Debug)]
pub enum Location<'arena>
{
    Distance(Distance),
    Mark(Id<'arena>),
    Expression(/* expresions to allow for like mark + 1 */),
}

#[derive(Clone, Copy, Debug)]
pub enum LaneSelection<'arena>
{
    Single(Id<'arena>),
    List(&'arena [LaneSelection<'arena>]),
    Range(Id<'arena>, Id<'arena>),
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
