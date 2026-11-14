use bumpalo::Bump;
use winnow::{
    LocatingSlice, ModalResult, Parser,
    ascii::multispace0,
    combinator::{delimited, preceded, repeat},
    error::{ContextError, ErrMode, ParserError},
    stream::{AsChar, Stream, StreamIsPartial},
};

use crate::parser::{statement::Statement, string::span_parse_string};

pub mod statement;
pub mod string;
pub mod lane;

type Span<'a> = LocatingSlice<&'a str>;
type In<'a> = Span<'a>;

type ParseResult<T> = ModalResult<T>;
type ErrorType = ErrMode<ContextError>;

pub struct Diagram<'a>
{
    name: String,
    statements: Vec<Statement<'a>>,
}

impl<'a> Diagram<'a>
{
    pub fn parser<'b>(bump: &'a Bump) -> impl Parser<Span<'b>, Self, ErrorType>
    {
        preceded(
            ws("diagram"),
            (
                span_parse_string,
                delimited(ws('{'), repeat(0.., Statement::parser(bump)), ws('}')),
            ),
        )
        .map(|(name, statements)| Self { name, statements })
    }
}

pub fn ws<I, O, E, G>(parser: G) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    E: ParserError<I>,
    G: Parser<I, O, E>,
{
    delimited(multispace0, parser, multispace0)
}
