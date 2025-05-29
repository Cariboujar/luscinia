//! Text formatter implementation

use crate::formatter::LocaleConfig;
use crate::formatter::error::FormatResult;
use crate::types::elements::*;
use crate::types::{AmPm, NFText};

/// Format text according to NFText format specification
pub fn format_text(value: &str, format: &NFText, locale: &LocaleConfig) -> FormatResult {
    let mut result = String::new();
    let mut at_replaced = false;

    for element in &format.elements {
        match element {
            TextFormatElement::AtPlaceholder => {
                // @ is the placeholder for the text value
                result.push_str(value);
                at_replaced = true;
            }
            TextFormatElement::AmPm(ampm) => {
                // In text format, AM/PM usually doesn't make sense but we can handle it
                // For consistency, we'll render it as is
                result.push_str(&format_ampm(ampm, false)); // Default to AM
            }
            TextFormatElement::LiteralCharSpace(c) => {
                result.push(' '); // Space character
                result.push(*c); // Following character
            }
            TextFormatElement::LiteralString(s) => {
                result.push_str(s);
            }
            TextFormatElement::FillChar(c) => {
                result.push(*c);
            }
            TextFormatElement::EscapedChar(c) => {
                result.push(*c);
            }
            TextFormatElement::BareChar(c) => {
                result.push(*c);
            }
        }
    }

    // If there was no @ placeholder in the format, append the text at the end
    if !at_replaced
        && format
            .elements
            .iter()
            .any(|e| matches!(e, TextFormatElement::AtPlaceholder))
    {
        result.push_str(value);
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
