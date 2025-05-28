use crate::types::common::*;
use crate::types::format_elements::*;
use serde::Serialize;

use super::AmPm;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFGeneral;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFNumber {
    pub num_part: Vec<DigitPosOrOther<Percent>>,
    pub exp_part: Option<(Sign, Vec<DigitPosOrOther<Percent>>)>,
    pub has_percent: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFFraction {
    pub numerator: Vec<FracToken>,
    pub denominator: Vec<FracToken>,
    pub integer_part: Option<Vec<DigitPosOrOther<Percent>>>,
    pub ampm_part: Vec<AmPm>,
}
