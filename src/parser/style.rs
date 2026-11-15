use winnow::{Parser, Result, combinator::{preceded, repeat, seq}, stream::AsChar, token::take_while};

use crate::parser::{In, ParseResult, statement::Id};

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
