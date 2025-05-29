//! Core implementation of the Excel numfmt formatter

use crate::formatter::LocaleConfig;
use crate::formatter::datetime_fmt::format_datetime;
use crate::formatter::error::FormatResult;
use crate::formatter::number_fmt::{
    format_fraction, format_nf_number, format_parenthesized_number,
};
use crate::formatter::text_fmt::format_text;
use crate::types::common::*;
use crate::types::elements::*;
use crate::types::number::*;
use crate::types::numfmt::*;

/// Format a value using a parsed NumFormat
pub fn format_with_parsed(
    value: f64,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    let locale = locale_config.unwrap_or_default();

    match format {
        NumFormat::ConditionalGeneral(section) => {
            format_conditional_general(value, section, &locale)
        }
        NumFormat::AnyNoCond(section) => format_any_no_cond(value, section, &locale),
        NumFormat::TwoParts(positive, negative) => {
            if value >= 0.0 {
                format_any_no_text(value, positive, &locale)
            } else {
                format_any(value.abs(), negative, &locale)
            }
        }
        NumFormat::ThreeParts(positive, negative, zero) => {
            if value > 0.0 {
                format_any_no_text(value, positive, &locale)
            } else if value < 0.0 {
                format_any_no_text(value.abs(), negative, &locale) // Use abs value for negative format
            } else {
                format_any_no_cond(value, zero, &locale)
            }
        }
        NumFormat::FourParts(positive, negative, zero, text) => {
            if value > 0.0 {
                format_any_no_text(value, positive, &locale)
            } else if value < 0.0 {
                format_any_no_text(value.abs(), negative, &locale) // Use abs value for negative format
            } else if let Some(text_fmt) = text {
                // This is for text values, but we're formatting a number
                // In Excel, the text format is only used for text values
                format_any_no_text_no_cond(value, zero, &locale)
            } else {
                format_any_no_text_no_cond(value, zero, &locale)
            }
        }
    }
}

/// Format a string value using a parsed NumFormat
pub fn format_string_with_parsed(
    value: &str,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    let locale = locale_config.unwrap_or_default();

    match format {
        NumFormat::FourParts(_, _, _, Some(text_fmt)) => {
            // Use the text format part for strings
            match text_fmt {
                TextOr::Text(text_section) => {
                    apply_bare_text_formatting(value, text_section.clone(), &locale)
                }
                TextOr::Other(_) => {
                    // If it's not a text format, just return the string
                    Ok(value.to_string())
                }
            }
        }
        _ => {
            // For other formats, just return the string
            Ok(value.to_string())
        }
    }
}

/// Format a value with a conditional general format
fn format_conditional_general(
    value: f64,
    section: &SectionWrapper<(NFPartCondition, NFGeneral)>,
    locale: &LocaleConfig,
) -> FormatResult {
    let (condition, _) = &section.inner;

    // Check if the condition is met
    if evaluate_condition(value, condition) {
        // Apply general format
        let formatted = format!("{}", value);
        apply_section_decorations(&formatted, section, locale)
    } else {
        // Condition not met, use default format
        Ok(format!("{}", value))
    }
}

/// Format a value with an unconditional format
fn format_any_no_cond(
    value: f64,
    section: &SectionWrapper<TextOr<NumberOrFracOrDt>>,
    locale: &LocaleConfig,
) -> FormatResult {
    match &section.inner {
        TextOr::Text(text) => {
            // Should not happen for numbers, but handle anyway
            let formatted = format_text(&value.to_string(), text, locale)?;
            apply_section_decorations(&formatted, section, locale)
        }
        TextOr::Other(number_or_frac_or_dt) => {
            let formatted = format_number_or_frac_or_dt(value, number_or_frac_or_dt, locale)?;
            apply_section_decorations(&formatted, section, locale)
        }
    }
}

/// Format a value with an AnyNoText format (no text allowed)
fn format_any_no_text(
    value: f64,
    section: &SectionWrapper<AnyInner>,
    locale: &LocaleConfig,
) -> FormatResult {
    match &section.inner {
        AnyInner::Data(number_or_frac_or_dt) => {
            let formatted = format_number_or_frac_or_dt(value, number_or_frac_or_dt, locale)?;
            apply_section_decorations(&formatted, section, locale)
        }
        AnyInner::ConditionalData(condition, number_or_frac_or_dt) => {
            // Check if condition is met
            if let Some(cond) = condition {
                if !evaluate_condition(value, cond) {
                    // Condition not met, use default format
                    return Ok(format!("{}", value));
                }
            }

            let formatted = format_number_or_frac_or_dt(value, number_or_frac_or_dt, locale)?;
            apply_section_decorations(&formatted, section, locale)
        }
    }
}

/// Format a value with an Any format (could be text or other)
fn format_any(value: f64, section: &Any, locale: &LocaleConfig) -> FormatResult {
    match section {
        Any::Text(text_section) => {
            // Value is a number but format is text
            let formatted = format_text(&value.to_string(), &text_section.inner, locale)?;
            apply_section_decorations(&formatted, text_section, locale)
        }
        Any::Other(other_section) => format_any_no_text(value, other_section, locale),
    }
}

/// Format a value with an AnyNoTextNoCond format (no text, no conditions)
fn format_any_no_text_no_cond(
    value: f64,
    section: &SectionWrapper<NumberOrFracOrDt>,
    locale: &LocaleConfig,
) -> FormatResult {
    let formatted = format_number_or_frac_or_dt(value, &section.inner, locale)?;
    apply_section_decorations(&formatted, section, locale)
}

/// Format a number/fraction/datetime value
fn format_number_or_frac_or_dt(
    value: f64,
    format: &NumberOrFracOrDt,
    locale: &LocaleConfig,
) -> FormatResult {
    match format {
        NumberOrFracOrDt::Number(number) => format_nf_number(value, number, locale),
        NumberOrFracOrDt::ParenthesizedNumber(number) => {
            format_parenthesized_number(value, number, locale)
        }
        NumberOrFracOrDt::Fraction(fraction) => format_fraction(value, fraction, locale),
        NumberOrFracOrDt::Datetime(datetime) => format_datetime(value, datetime, locale),
    }
}

/// Apply section wrapper decorations (color, locale, etc.) to a formatted string
fn apply_section_decorations<T>(
    formatted: &str,
    section: &SectionWrapper<T>,
    locale: &LocaleConfig,
) -> FormatResult {
    let mut result = formatted.to_string();

    // Apply color if present
    if let Some(color) = &section.color {
        // In real implementation, this would apply color formatting
        // For now, we'll just add a color indicator
        result = format_with_color(&result, color);
    }

    // Apply locale if present
    if let Some(locale_id) = &section.locale {
        // In real implementation, this would apply locale-specific formatting
        // For now, we'll just add the currency symbol if present
        if !locale_id.currency_symbol.is_empty() {
            result = format!("{}{}", locale_id.currency_symbol, result);
        }
    }

    // Apply Thai year prefix if present
    if section.is_thai_prefixed {
        // In real implementation, this would convert to Thai year
        // For now, we'll just add a prefix
        result = format!("[THAI]{}", result);
    }

    Ok(result)
}

/// Apply section wrapper for text formats
fn apply_section_wrapper_text(
    value: &str,
    section: &SectionWrapper<NFText>,
    locale: &LocaleConfig,
) -> FormatResult {
    let formatted = format_text(value, &section.inner, locale)?;
    apply_section_decorations(&formatted, section, locale)
}

/// Apply formatting for a bare NFText (not wrapped in SectionWrapper)
fn apply_bare_text_formatting(value: &str, text: NFText, locale: &LocaleConfig) -> FormatResult {
    let wrapper = SectionWrapper {
        is_thai_prefixed: false,
        locale: None,
        color: None,
        inner: text,
    };
    apply_section_wrapper_text(value, &wrapper, locale)
}

/// Format a string with color
fn format_with_color(value: &str, color: &NFPartColor) -> String {
    match color {
        NFPartColor::Intl(defined_color) => {
            format!("[{}]{}", format_defined_color(defined_color), value)
        }
        NFPartColor::Color(index) => {
            format!("[Color{}]{}", index, value)
        }
    }
}

/// Format a defined color
fn format_defined_color(color: &DefinedColor) -> String {
    match color {
        DefinedColor::Black => "Black".to_string(),
        DefinedColor::Blue => "Blue".to_string(),
        DefinedColor::Cyan => "Cyan".to_string(),
        DefinedColor::Green => "Green".to_string(),
        DefinedColor::Magenta => "Magenta".to_string(),
        DefinedColor::Red => "Red".to_string(),
        DefinedColor::White => "White".to_string(),
        DefinedColor::Yellow => "Yellow".to_string(),
    }
}

/// Evaluate a condition against a value
fn evaluate_condition(value: f64, condition: &NFPartCondition) -> bool {
    match condition.op {
        NFCondOperator::Equal => value == condition.value,
        NFCondOperator::GreaterThan => value > condition.value,
        NFCondOperator::LessThan => value < condition.value,
        NFCondOperator::GreaterThanOrEqual => value >= condition.value,
        NFCondOperator::LessThanOrEqual => value <= condition.value,
    }
}
