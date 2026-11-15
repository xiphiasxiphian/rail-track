use bumpalo::Bump;
use winnow::{Parser, Result, ascii::float, combinator::{alt, dispatch, empty, fail, preceded, repeat, seq}, stream::AsChar, token::take_while};

use crate::parser::{ErrorType, In, ParseResult, block, kw, kw_slice, statement::{Id, Statement}, ws};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Colour
{
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer
{
    Below,
    Above,
}

#[derive(Clone, Copy, Debug)]
pub enum StyleAttribute
{
    Stroke(Colour),
    Fill(Colour),
    Width(f64),
    Dash(f64),
    Layer(Layer),
}

#[derive(Clone, Debug)]
pub struct Style<'arena>
{
    id: Id<'arena>,
    attributes: &'arena [StyleAttribute],
}

impl Colour
{
    pub fn parse(input: &mut In) -> ParseResult<Self>
    {
        seq!(
            Colour {
                _: '#',
                r: Self::digit,
                g: Self::digit,
                b: Self::digit,
            }
        ).parse_next(input)
    }

    fn digit<'a, 'b>(input: &'b mut In<'a>) -> ParseResult<u8>
    {
        take_while(2, AsChar::is_hex_digit)
            .try_map(|s: &str| u8::from_str_radix(s, 16))
            .parse_next(input)
    }
}

impl StyleAttribute
{
    fn parse(input: &mut In) -> ParseResult<Self>
    {
        dispatch! {kw_slice;
            "stroke" => ws(Colour::parse).map(Self::Stroke),
            "fill" => ws(Colour::parse).map(Self::Fill),
            "width" => ws(float).map(Self::Width),
            "dash" => ws(float).map(Self::Dash),
            "layer" => ws(
                dispatch!{
                    kw_slice;
                    "below" => empty.value(Layer::Below),
                    "above" => empty.value(Layer::Above),
                    _ => fail
                }
            ).map(Self::Layer),
            _  => fail // TODO: better errors here
        }.parse_next(input)
    }
}

impl<'arena> Style<'arena>
{
    pub fn parser<'a>(bump: &'arena Bump) -> impl Parser<In<'a>, Self, ErrorType>
    {
        seq!(Self {
            _: kw("style"),
            id: Statement::id_parser(bump),
            attributes: block(repeat(0.., StyleAttribute::parse))
                .map(|ps: Vec<StyleAttribute>| bump.alloc_slice_copy(&ps))
        })
    }
}
