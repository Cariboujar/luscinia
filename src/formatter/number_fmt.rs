//! Number formatter implementation

use crate::formatter::LocaleConfig;
use crate::formatter::error::{FormatError, FormatResult};
use crate::types::common::*;
use crate::types::datetime::*;
use crate::types::elements::*;
use crate::types::number::*;

/// Format a number according to a NFNumber format specification
pub fn format_nf_number(value: f64, format: &NFNumber, locale: &LocaleConfig) -> FormatResult {
    let mut result = String::new();
    let decimal_sep = locale.decimal_separator.unwrap_or('.');
    let thousands_sep = locale.thousands_separator.unwrap_or(',');

    // Check if value is negative - we'll handle the sign separately
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    let formatting_value = if format.has_percent {
        abs_value * 100.0
    } else {
        abs_value
    };

    if let Some((sign, exp_part)) = &format.exp_part {
        let (mantissa, exponent) = scientific_decompose(formatting_value);
        let mantissa_str =
            format_number_part(mantissa, &format.num_part, decimal_sep, thousands_sep)?;
        result.push_str(&mantissa_str);

        result.push('E');
        if matches!(sign, Sign::Plus) || exponent < 0 {
            result.push(if exponent < 0 { '-' } else { '+' });
        } else {
            result.push('+');
        }

        let exp_str =
            format_number_part(exponent.abs() as f64, exp_part, decimal_sep, thousands_sep)?;
        result.push_str(&exp_str);
    } else {
        result = format_number_part(
            formatting_value,
            &format.num_part,
            decimal_sep,
            thousands_sep,
        )?;
    }

    // Add percent sign if needed
    if format.has_percent {
        result.push('%');
    }

    // Handle negative sign
    if is_negative {
        result = format!("-{}", result);
    }

    Ok(result)
}

/// Format a number with parentheses for negative values
pub fn format_parenthesized_number(
    value: f64,
    format: &NFNumber,
    locale: &LocaleConfig,
) -> FormatResult {
    let result = format_nf_number(value.abs(), format, locale)?;
    Ok(format!("({})", result))
}

/// Format a fraction according to NFFraction format specification
pub fn format_fraction(value: f64, format: &NFFraction, locale: &LocaleConfig) -> FormatResult {
    let mut result = String::new();

    // Extract integer and fractional parts
    let integer_part = value.trunc();
    let fractional_part = value.abs() - integer_part.abs();

    // Format integer part if present in the format
    if let Some(int_format) = &format.integer_part {
        let int_str = format_number_part(
            integer_part.abs(),
            int_format,
            locale.decimal_separator.unwrap_or('.'),
            locale.thousands_separator.unwrap_or(','),
        )?;
        result.push_str(&int_str);
        result.push(' '); // Space between integer and fraction
    }

    // Determine numerator and denominator
    let (numerator, denominator) = convert_to_fraction(fractional_part);

    // Format numerator
    let num_str = format_fraction_part(numerator, &format.numerator)?;

    // Format denominator
    let denom_str = format_fraction_part(denominator, &format.denominator)?;

    // Combine as a fraction
    result.push_str(&format!("{}/{}", num_str, denom_str));

    // Add AM/PM if present
    if !format.ampm_part.is_empty() {
        result.push(' ');
        for ampm in &format.ampm_part {
            result.push_str(&format_ampm(ampm, value as i64 >= 12));
        }
    }

    // Handle negative sign if integer part isn't formatted separately
    if value < 0.0 && format.integer_part.is_none() {
        result = format!("-{}", result);
    }

    Ok(result)
}

/// Helper function to convert a decimal to a fraction
fn convert_to_fraction(value: f64) -> (i64, i64) {
    // This is a simplified implementation
    // In a real implementation, you would use a more robust algorithm
    // to find the best approximation of the decimal as a fraction

    const MAX_DENOMINATOR: i64 = 1000000;
    let mut numerator = (value * MAX_DENOMINATOR as f64).round() as i64;
    let mut denominator = MAX_DENOMINATOR;

    // Simplify fraction using GCD
    let gcd = gcd(numerator, denominator);
    numerator /= gcd;
    denominator /= gcd;

    (numerator, denominator)
}

/// Calculate Greatest Common Divisor using Euclidean algorithm
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

/// Format a number part (either main part or exponent)
fn format_number_part(
    value: f64,
    format_parts: &[DigitPosOrOther<Percent>],
    decimal_sep: char,
    thousands_sep: char,
) -> FormatResult {
    let mut int_format = Vec::new();
    let mut dec_format = Vec::new();
    let mut has_decimal = false;
    let mut use_thousands = false;

    for part in format_parts {
        match part {
            DigitPosOrOther::Digit(DigitPos::Separator(NumSeparator::Decimal)) => {
                has_decimal = true;
            }
            DigitPosOrOther::Digit(DigitPos::Separator(NumSeparator::NumberGroup)) => {
                use_thousands = true;
                if !has_decimal {
                    int_format.push(part.clone());
                } else {
                    dec_format.push(part.clone());
                }
            }
            _ => {
                if !has_decimal {
                    int_format.push(part.clone());
                } else {
                    dec_format.push(part.clone());
                }
            }
        }
    }

    let dec_digits = if has_decimal {
        dec_format
            .iter()
            .filter(|part| matches!(part, DigitPosOrOther::Digit(DigitPos::Digit(_))))
            .count()
    } else {
        0
    };

    let rounded_value = if dec_digits > 0 {
        let factor = 10f64.powi(dec_digits as i32);
        (value * factor).round() / factor
    } else {
        value.round()
    };

    let int_value = rounded_value.trunc() as i64;
    let frac_value = (rounded_value - int_value as f64).abs();

    let mut int_digits = 0;
    let mut dec_digits = 0;

    for part in &int_format {
        if let DigitPosOrOther::Digit(DigitPos::Digit(_)) = part {
            int_digits += 1;
        }
    }

    for part in &dec_format {
        if let DigitPosOrOther::Digit(DigitPos::Digit(_)) = part {
            dec_digits += 1;
        }
    }

    let mut int_result = String::new();
    let int_str = int_value.to_string();

    // not enough digits to show full number, add extra digits
    // should calculate the thousands separator
    if int_digits < int_str.len() && int_digits > 0 {
        for i in 0..(int_str.len() - int_digits) {
            int_result.push(int_str.chars().nth(i).unwrap());
            if use_thousands && (int_str.len() - i - 1) % 3 == 0 && i < int_str.len() - 1 {
                int_result.push(thousands_sep);
            }
        }
    }

    let mut digit_pos = 0;

    if value.abs() < 1.0 {
        match int_format.last() {
            Some(DigitPosOrOther::Digit(DigitPos::Digit(placeholder))) => match placeholder {
                NumPlaceholder::Zero => int_result.push('0'),
                NumPlaceholder::Space => int_result.push(' '),
                NumPlaceholder::Lazy => {}
            },
            Some(DigitPosOrOther::LiteralString(s)) => {
                int_result.push_str(s);
            }
            Some(DigitPosOrOther::LiteralCharSpace(_c)) => {
                // width should be same as `c`, however as string
                // we can only push one space
                int_result.push(' ');
            }
            Some(DigitPosOrOther::FillChar(c)) => {
                int_result.push(*c);
            }
            Some(DigitPosOrOther::EscapedChar(c)) => {
                int_result.push(*c);
            }
            _ => {}
        }
    } else {
        for part in &int_format {
            match part {
                DigitPosOrOther::Digit(DigitPos::Digit(placeholder)) => {
                    // rtl
                    let pos = int_digits - digit_pos - 1;
                    let digit_idx = int_str.len() as isize - pos as isize - 1;

                    if digit_idx >= 0 && digit_idx < int_str.len() as isize {
                        if int_str.len() < int_digits
                            || digit_idx as usize >= int_str.len() - int_digits
                        {
                            int_result.push(int_str.chars().nth(digit_idx as usize).unwrap());
                        }
                        if use_thousands
                            && (int_str.len() - digit_idx as usize - 1) % 3 == 0
                            && digit_idx < int_str.len() as isize - 1
                        {
                            int_result.push(thousands_sep);
                        }
                    } else if digit_idx < 0 {
                        // 如果数字位数不够，用 0 补足
                        match placeholder {
                            NumPlaceholder::Zero => {
                                int_result.push('0');
                                if use_thousands
                                    && (int_str.len() - digit_idx as usize - 1) % 3 == 0
                                    && digit_idx < int_str.len() as isize - 1
                                {
                                    int_result.push(thousands_sep);
                                }
                            }
                            NumPlaceholder::Space => int_result.push(' '),
                            NumPlaceholder::Lazy => {}
                        }
                    }
                    digit_pos += 1;
                }
                DigitPosOrOther::LiteralString(s) => {
                    int_result.push_str(s);
                }
                DigitPosOrOther::LiteralCharSpace(_c) => {
                    // width should be same as `c`, however as string
                    // we can only push one space
                    int_result.push(' ');
                }
                DigitPosOrOther::FillChar(c) => {
                    int_result.push(*c);
                }
                DigitPosOrOther::EscapedChar(c) => {
                    int_result.push(*c);
                }
                _ => {}
            }
        }
    }

    let mut dec_result = String::new();
    let dec_str = if dec_digits > 0 && frac_value != 0.0 {
        format!("{:.*}", dec_digits, frac_value)
            .trim_start_matches("0")
            .trim_start_matches(".")
            .to_string()
    } else {
        String::new()
    };

    let mut digit_pos = 0;
    for part in &dec_format {
        match part {
            DigitPosOrOther::Digit(DigitPos::Digit(placeholder)) => {
                if digit_pos < dec_str.len() {
                    dec_result.push(dec_str.chars().nth(digit_pos).unwrap());
                } else {
                    match placeholder {
                        NumPlaceholder::Zero => dec_result.push('0'),
                        NumPlaceholder::Space => dec_result.push(' '),
                        NumPlaceholder::Lazy => {} // # 不显示
                    }
                }
                digit_pos += 1;
            }
            DigitPosOrOther::LiteralString(s) => {
                dec_result.push_str(s);
            }
            DigitPosOrOther::LiteralCharSpace(c) => {
                dec_result.push(' ');
                dec_result.push(*c);
            }
            DigitPosOrOther::FillChar(c) => {
                dec_result.push(*c);
            }
            DigitPosOrOther::EscapedChar(c) => {
                dec_result.push(*c);
            }
            _ => {}
        }
    }

    let mut result = String::new();
    result.push_str(&int_result);
    if has_decimal && (!dec_result.is_empty() || dec_digits > 0) {
        result.push(decimal_sep);
        result.push_str(&dec_result);
    }

    Ok(result)
}

/// Format a fraction part (numerator or denominator)
fn format_fraction_part(value: i64, format_parts: &[FracToken]) -> FormatResult {
    let value_str = value.to_string();
    let mut result = String::new();

    // Count digit placeholders
    let digit_count = format_parts
        .iter()
        .filter(|token| matches!(token, FracToken::Placeholder(_)))
        .count();

    // If value has more digits than the format can accommodate, return error
    if value_str.len() > digit_count && digit_count > 0 {
        return Err(FormatError::FormatError(format!(
            "Value {} has more digits than format can accommodate ({})",
            value, digit_count
        )));
    }

    let mut value_index = 0;

    // Build the formatted fraction part
    for token in format_parts {
        match token {
            FracToken::Placeholder(placeholder) => {
                let offset = value_str.len() as isize - digit_count as isize + value_index as isize;

                if offset >= 0 && offset < value_str.len() as isize {
                    result.push(value_str.chars().nth(offset as usize).unwrap());
                } else {
                    // Handle placeholders based on type
                    match placeholder {
                        NumPlaceholder::Zero => result.push('0'),
                        NumPlaceholder::Space => result.push(' '),
                        NumPlaceholder::Lazy => {} // Skip for # placeholder
                    }
                }

                value_index += 1;
            }
            FracToken::Percent => {
                result.push('%');
            }
        }
    }

    Ok(result)
}

/// Break a number into scientific notation parts (mantissa and exponent)
fn scientific_decompose(value: f64) -> (f64, i32) {
    if value == 0.0 {
        return (0.0, 0);
    }

    let exp = value.abs().log10().floor() as i32;
    let mantissa = value / 10f64.powi(exp);

    (mantissa, exp)
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
