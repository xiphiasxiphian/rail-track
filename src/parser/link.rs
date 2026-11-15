use bumpalo::Bump;
use winnow::{Parser, combinator::{alt, dispatch, empty, fail, opt, preceded, seq}, error::{StrContext, StrContextValue}};

use crate::parser::{ErrorType, In, ParseResult, kw, kw_slice, lane::LaneSelection, statement::{Id, Location, Statement}, ws};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkKind
{
    Link,
    Branch,
    Merge,
    Crossover,
    Scissors,
    Diamond,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Grade
{
    Under,
    #[default] Level,
    Over,
}

#[derive(Clone, Copy, Debug)]
pub struct Link<'arena>
{
    kind: LinkKind,
    from: LaneSelection<'arena>,
    to: LaneSelection<'arena>,
    bidirectional: bool,

    start: Option<Location<'arena>>,
    end: Option<Location<'arena>>,

    grade: Grade,
    name: Option<Id<'arena>>
}

impl<'arena> Link<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        seq!(
            Self {
                kind: Self::parse_kind,
                from: ws(LaneSelection::parser(bump)),
                bidirectional: ws(alt(("<->".value(true), "->".value(false)))),
                to: ws(LaneSelection::parser(bump)),
                start: ws(opt(preceded(kw("at"), ws(Location::parser(bump))))),
                end: ws(opt(preceded(kw("to"), ws(Location::parser(bump))))),
                grade: ws(opt(alt((
                    kw("under").value(Grade::Under),
                    kw("level").value(Grade::Level),
                    kw("over").value(Grade::Over),
                ))).map(|x| x.unwrap_or_default())),
                name: opt(preceded(kw("as"), ws(Statement::id_parser(bump)))),
            }
        )
    }

    fn parse_kind(input: &mut In) -> ParseResult<LinkKind>
    {
        dispatch! {
            kw_slice;
            "link" => empty.value(LinkKind::Link),
            "branch" => empty.value(LinkKind::Branch),
            "merge" => empty.value(LinkKind::Merge),
            "crossover" => empty.value(LinkKind::Crossover),
            "scissors" => empty.value(LinkKind::Scissors),
            "diamond" => empty.value(LinkKind::Diamond),
            _ => fail.context(StrContext::Expected(StrContextValue::Description("a link type"))),
        }.parse_next(input)
    }
}
