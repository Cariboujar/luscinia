use crate::types::datetime::AmPm;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TextFormatElement {
    AtPlaceholder,
    AmPm(AmPm),
    LiteralString(String),
    LiteralCharSpace(char),
    FillChar(char),
    EscapedChar(char),
    BareChar(char),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFText {
    pub elements: Vec<TextFormatElement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum NumPlaceholder {
    /// 0
    Zero,
    /// #
    Lazy,
    /// ?
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[repr(u8)]
pub enum NumSeparator {
    /// .
    Decimal,
    /// ,
    NumberGroup,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DigitPos {
    Digit(NumPlaceholder),
    Separator(NumSeparator),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum DigitPosOrOther<T> {
    Digit(DigitPos),
    Other(T),
    LiteralCharSpace(char),
    LiteralString(String),
    FillChar(char),
    EscapedChar(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Percent;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FracToken {
    Placeholder(NumPlaceholder),
    Percent,
}
