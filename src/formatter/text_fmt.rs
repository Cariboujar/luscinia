//! Text formatter implementation

use crate::formatter::LocaleConfig;
use crate::formatter::error::FormatResult;
use crate::types::elements::*;
use crate::types::{AmPm, NFText};

/// Format text according to NFText format specification
pub fn format_text(value: &str, format: &NFText, _locale: &LocaleConfig) -> FormatResult {
    let mut result = String::new();

    for element in &format.elements {
        match element {
            TextFormatElement::AtPlaceholder => {
                // @ is the placeholder for the text value
                result.push_str(value);
            }
            TextFormatElement::AmPm(ampm) => {
                // In text format, AM/PM usually doesn't make sense but we can handle it
                // For consistency, we'll render it as is
                result.push_str(&format_ampm(ampm, false)); // Default to AM
            }
            TextFormatElement::LiteralCharSpace(_c) => {
                // the width should be same as c but we can't handle it in string
                result.push(' ');
            }
            TextFormatElement::LiteralString(s) => {
                result.push_str(s);
            }
            TextFormatElement::FillChar(c) => {
                // repeat the character 5 times, should be able to configure the number of times
                const COUNT: usize = 5;
                for _ in 0..COUNT {
                    result.push(*c);
                }
            }
            TextFormatElement::EscapedChar(c) => {
                result.push(*c);
            }
            TextFormatElement::BareChar(c) => {
                result.push(*c);
            }
        }
    }

    Ok(result)
}

/// Format AM/PM indicator
fn format_ampm(ampm: &AmPm, is_pm: bool) -> String {
    match ampm {
        AmPm::Full => {
            if is_pm {
                "PM".to_string()
            } else {
                "AM".to_string()
            }
        }
        AmPm::Simple => {
            if is_pm {
                "P".to_string()
            } else {
                "A".to_string()
            }
        }
    }
}
