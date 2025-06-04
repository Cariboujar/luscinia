use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SectionWrapper<T> {
    pub locale: Option<PartLocaleID>,
    pub color: Option<NFPartColor>,
    pub special_prefix: Vec<String>,  // 统一存储 "t", "ENG", "DBNum1", "HIJ", "JPN", "TWN" 等
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TextOr<T> {
    Text(NFText),
    Other(T),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[repr(u8)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[repr(u8)]
pub enum DefinedColor {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum NFPartColor {
    Intl(DefinedColor),
    Color(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[repr(u8)]
pub enum NFCondOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// [>=1.0]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFPartCondition {
    pub op: NFCondOperator,
    pub value: f64,
}

impl Eq for NFPartCondition {}

// Forward references
use crate::types::elements::NFText;
use crate::types::locale::PartLocaleID;
