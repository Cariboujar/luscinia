use crate::types::common::*;
use crate::types::datetime::*;
use crate::types::elements::*;
use crate::types::number::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NumFormat {
    ConditionalGeneral(SectionWrapper<(NFPartCondition, NFGeneral)>),
    AnyNoCond(AnyNoCond),
    TwoParts(AnyNoText, Any),
    ThreeParts(AnyNoText, AnyNoText, AnyNoCond),
    FourParts(
        AnyNoText,
        AnyNoText,
        AnyNoTextNoCond,
        Option<TextOr<NFGeneral>>,
    ),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AnyInner {
    Data(NumberOrFracOrDt),
    ConditionalData(Option<NFPartCondition>, NumberOrFracOrDt),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Any {
    Text(SectionWrapper<NFText>),
    Other(SectionWrapper<AnyInner>),
}

pub type AnyNoText = SectionWrapper<AnyInner>;
pub type AnyNoCond = SectionWrapper<TextOr<NumberOrFracOrDt>>;
pub type AnyNoTextNoCond = SectionWrapper<NumberOrFracOrDt>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NumberOrFracOrDt {
    Number(NFNumber),
    ParenthesizedNumber(NFNumber),
    Fraction(NFFraction),
    Datetime(DatetimeTuple),
}
