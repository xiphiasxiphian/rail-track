use bumpalo::Bump;
use winnow::{
    LocatingSlice, ModalResult, Parser, ascii::multispace0, combinator::{delimited, not, preceded, repeat, terminated}, error::{ContextError, ErrMode, ParserError, StrContext, StrContextValue}, stream::{AsChar, Compare, Stream, StreamIsPartial}, token::{literal, one_of, take_while},
};

use crate::parser::{statement::Statement, string::span_parse_string};

pub mod statement;
pub mod string;
pub mod lane;
pub mod style;

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

pub fn kw<'a>(word: &'static str) -> impl Parser<In<'a>, (), ErrorType>
{
    terminated(word, not(one_of(|c: char| c.is_alphanumeric() || c == '_'))).void()
}

pub fn kw_slice<'a>(input: &mut In<'a>) -> ParseResult<&'a str>
{
    take_while(1.., |c: char| c.is_alphabetic() || c == '_').parse_next(input)
}

// very handy for making better error messages
pub fn closing_brace<'a>() -> impl Parser<In<'a>, (), ErrorType>
{
    ws('}')
        .void()
        .context(StrContext::Expected(StrContextValue::Description("a statement or '}'")))
}

pub fn block<'a, O, G>(parser: G) -> impl Parser<In<'a>, O, ErrorType>
where
    G: Parser<In<'a>, O, ErrorType>,
{
    delimited(ws('{'), parser, closing_brace())
}
