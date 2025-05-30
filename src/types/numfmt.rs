use crate::types::common::*;
use crate::types::datetime::*;
use crate::types::elements::*;
use crate::types::number::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NumFormat {
    ConditionalGeneral(SectionWrapper<(NFPartCondition, NFGeneral)>),
    AnyNoCond(AnyNoCond),
    TwoParts(Any, Any),
    ThreeParts(Any, Any, AnyNoCond),
    FourParts(Any, Any, AnyNoCond, Option<NumberOrFracOrDtOrText>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AnyInner {
    ConditionalData(Option<NFPartCondition>, NumberOrFracOrDtOrText),
}

pub type Any = SectionWrapper<AnyInner>;
pub type AnyNoText = SectionWrapper<AnyInner>;
pub type AnyNoCond = SectionWrapper<NumberOrFracOrDtOrText>;
pub type AnyNoTextNoCond = SectionWrapper<NumberOrFracOrDtOrText>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum NumberOrFracOrDtOrText {
    General(),
    Number(NFNumber),
    ParenthesizedNumber(NFNumber),
    Fraction(NFFraction),
    Datetime(DatetimeTuple),
    Text(NFText),
}
