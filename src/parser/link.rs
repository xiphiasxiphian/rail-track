use crate::parser::{lane::LaneSelection, statement::{Id, Location}};

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
