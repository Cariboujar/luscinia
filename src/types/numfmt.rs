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
    FourParts(Any, Any, AnyNoCond, Option<FormatComponent>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AnyInner {
    ConditionalData(Option<NFPartCondition>, FormatComponent),
}

pub type Any = SectionWrapper<AnyInner>;
pub type AnyNoText = SectionWrapper<AnyInner>;
pub type AnyNoCond = SectionWrapper<FormatComponent>;
pub type AnyNoTextNoCond = SectionWrapper<FormatComponent>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum FormatComponent {
    General(),
    Number(NFNumber),
    Fraction(NFFraction),
    Datetime(DatetimeTuple),
    Text(NFText),
}
