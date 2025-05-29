//! Excel-like number format implementation

mod datetime_fmt;
mod error;
mod impl_fmt;
mod number_fmt;
mod text_fmt;
mod value;

pub use error::{FormatError, FormatResult};
pub use value::FormatValue;

use crate::parser::NumfmtParser;
use crate::types::NumFormat;

/// Locale configuration for number formatting
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LocaleConfig {
    /// Decimal separator character
    pub decimal_separator: Option<char>,
    /// Thousands separator character
    pub thousands_separator: Option<char>,
    /// Date format locale
    pub date_locale: Option<String>,
    /// Currency symbol
    pub currency_symbol: Option<String>,
}

/// Formats a value using the specified format string and optional locale configuration
pub fn format<T: Into<FormatValue>>(
    value: T,
    format_str: &str,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    let format_value = value.into();
    let format = parse_format_string(format_str)?;
    format_with_parsed(format_value, &format, locale_config)
}

/// Parse a format string into a NumFormat
fn parse_format_string(format_str: &str) -> Result<NumFormat, FormatError> {
    NumfmtParser::new(format_str)
        .parse()
        .map_err(|e| FormatError::ParseError(e.to_string()))
}

/// Format a value using a parsed NumFormat
fn format_with_parsed(
    value: FormatValue,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    match value {
        FormatValue::Number(num) => format_number(num, format, locale_config),
        FormatValue::String(s) => format_string(s, format, locale_config),
        FormatValue::Boolean(b) => format_boolean(b, format, locale_config),
    }
}

/// Format a number using the specified NumFormat
fn format_number(
    value: f64,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    impl_fmt::format_with_parsed(value, format, locale_config)
}

/// Format a string using the specified NumFormat
fn format_string(
    value: String,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    impl_fmt::format_string_with_parsed(&value, format, locale_config)
}

/// Format a boolean using the specified NumFormat
fn format_boolean(
    value: bool,
    format: &NumFormat,
    locale_config: Option<LocaleConfig>,
) -> FormatResult {
    // Convert boolean to number (1.0 for true, 0.0 for false) and use number formatter
    // This matches Excel's behavior for boolean values
    let num_value = if value { 1.0 } else { 0.0 };
    impl_fmt::format_with_parsed(num_value, format, locale_config)
}

#[cfg(test)]
mod tests {
    use super::impl_fmt::format_string_with_parsed;
    use super::{super::*, format_boolean};
    use crate::formatter::impl_fmt::format_with_parsed;
    use crate::parser::NumfmtParser;

    // Helper function to format a value with a format string
    fn test_format(value: impl Into<FormatValue>, format_str: &str) -> FormatResult {
        let format = NumfmtParser::new(format_str)
            .parse()
            .map_err(|e| FormatError::ParseError(e.to_string()))?;
        match value.into() {
            FormatValue::Number(num) => format_with_parsed(num, &format, None),
            FormatValue::String(num) => format_string_with_parsed(&num, &format, None),
            FormatValue::Boolean(num) => format_boolean(num, &format, None),
        }
    }

    #[test]
    fn test_basic_number_formats() {
        assert_eq!(test_format(1234.567, "#,##0.00").unwrap(), "1,234.57");
        assert_eq!(test_format(1234.567, "0.00").unwrap(), "1234.57");
        assert_eq!(test_format(1234.567, "#,##0").unwrap(), "1,235");
        assert_eq!(test_format(1.2345, "0.000").unwrap(), "1.235");
        assert_eq!(test_format(0.567, "0.00").unwrap(), "0.57");
        assert_eq!(test_format(0.567, ".00").unwrap(), ".57");
        assert_eq!(test_format(0.567, "0.0").unwrap(), "0.6");
        assert_eq!(test_format(0.567, "#.#").unwrap(), ".6");
        assert_eq!(test_format(1, "#.#").unwrap(), "1.");
        assert_eq!(test_format(0, "#.#").unwrap(), ".");
    }

    #[test]
    fn test_number_with_text() {
        assert_eq!(
            test_format(1234.567, "#,##0.00\" 元\"").unwrap(),
            "1,234.57 元"
        );
        assert_eq!(
            test_format(1234.567, "\"人民币 \"#,##0.00").unwrap(),
            "人民币 1,234.57"
        );
        assert_eq!(test_format(1234.567, "\"$\"#,##0.00").unwrap(), "$1,234.57");
        assert_eq!(
            test_format(1234.567, "\"总计：\"#,##0.00\" 元\"").unwrap(),
            "总计：1,234.57 元"
        );
    }

    #[test]
    fn test_percentage_formats() {
        // Percentage formatting
        assert_eq!(test_format(0.12345, "0.00%").unwrap(), "12.35%");
        assert_eq!(test_format(0.12345, "0%").unwrap(), "12%");
        assert_eq!(test_format(1.2345, "0.00%").unwrap(), "123.45%");
        assert_eq!(test_format(0.005, "0.00%").unwrap(), "0.50%");
    }

    #[test]
    fn test_scientific_notation() {
        println!("Test scientific notation");
        assert_eq!(test_format(12345.67, "0.00E+00").unwrap(), "1.23E+04");
        assert_eq!(test_format(0.00012345, "0.00E+00").unwrap(), "1.23E-04");
        assert_eq!(test_format(123.45, "0.00E-00").unwrap(), "1.23E+02");
        assert_eq!(test_format(0.012345, "0.000E+000").unwrap(), "1.235E-002");
    }

    #[test]
    fn test_parenthesized_numbers() {
        println!("Test parenthesized numbers");
        // 测试带括号的数字格式 - 应该始终使用括号，无论数字是正还是负
        assert_eq!(test_format(1234.56, "(#,##0.00)").unwrap(), "(1,234.56)");
        assert_eq!(test_format(-1234.56, "(#,##0.00)").unwrap(), "(1,234.56)");
    }

    #[test]
    fn test_multiple_section_formats() {
        // Multiple section formats (positive;negative;zero;text)
        assert_eq!(
            test_format(1234.56, "#,##0.00;(#,##0.00)").unwrap(),
            "1,234.56"
        );
        assert_eq!(
            test_format(-1234.56, "#,##0.00;(#,##0.00)").unwrap(),
            "(1,234.56)"
        );
        assert_eq!(
            test_format(0, "#,##0.00;(#,##0.00);\"Zero\"").unwrap(),
            "Zero"
        );

        assert_eq!(
            test_format(1234.56, "#,##0.00;(#,##0.00);\"Zero\";@").unwrap(),
            "1,234.56"
        );
        assert_eq!(
            test_format(-1234.56, "#,##0.00;(#,##0.00);\"Zero\";@").unwrap(),
            "(1,234.56)"
        );
        assert_eq!(
            test_format(0, "#,##0.00;(#,##0.00);\"Zero\";@").unwrap(),
            "Zero"
        );

        // Testing with string value should use the text format (4th section)
        assert_eq!(
            test_format("HelloWorld", "#,##0.00;(#,##0.00);\"Zero\";@").unwrap(),
            "HelloWorld"
        );
    }

    #[test]
    fn test_date_formats() {
        // TODO: LLM Generated, haven't been reviewed.

        let excel_date = 45061.0; // Excel serial date for May 15, 2023

        assert_eq!(test_format(excel_date, "yyyy-mm-dd").unwrap(), "2023-05-15");
        assert_eq!(test_format(excel_date, "m/d/yyyy").unwrap(), "5/15/2023");
        assert_eq!(
            test_format(excel_date, "mmm d, yyyy").unwrap(),
            "May 15, 2023"
        );
        assert_eq!(
            test_format(excel_date, "mmmm d, yyyy").unwrap(),
            "May 15, 2023"
        );
        assert_eq!(test_format(excel_date, "d-mmm-yy").unwrap(), "15-May-23");
        assert_eq!(test_format(excel_date, "d-mmm").unwrap(), "15-May");
        assert_eq!(test_format(excel_date, "mmm-yy").unwrap(), "May-23");
        assert_eq!(test_format(excel_date, "mmmm yy").unwrap(), "May 23");
        assert_eq!(
            test_format(excel_date, "m/d/yy h:mm").unwrap(),
            "5/15/23 0:00"
        );
        assert_eq!(
            test_format(excel_date, "dddd, mmmm dd, yyyy").unwrap(),
            "Monday, May 15, 2023"
        );
    }

    #[test]
    fn test_time_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Time formatting - using a fixed time value representing 15:30:45
        let excel_time = 0.647049; // Excel time value for 15:30:45

        assert_eq!(test_format(excel_time, "h:mm").unwrap(), "15:30");
        assert_eq!(test_format(excel_time, "h:mm:ss").unwrap(), "15:30:45");
        assert_eq!(test_format(excel_time, "h:mm AM/PM").unwrap(), "3:30 PM");
        assert_eq!(
            test_format(excel_time, "h:mm:ss AM/PM").unwrap(),
            "3:30:45 PM"
        );
        assert_eq!(test_format(excel_time, "[h]:mm:ss").unwrap(), "15:30:45");
        assert_eq!(test_format(excel_time, "mm:ss").unwrap(), "30:45");
        assert_eq!(
            test_format(excel_time, "h \"小时\" m \"分钟\"").unwrap(),
            "15 小时 30 分钟"
        );
    }

    #[test]
    fn test_datetime_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Combined date and time - using May 15, 2023 15:30:45
        let excel_datetime = 45061.647049;

        assert_eq!(
            test_format(excel_datetime, "yyyy-mm-dd hh:mm:ss").unwrap(),
            "2023-05-15 15:30:45"
        );
        assert_eq!(
            test_format(excel_datetime, "m/d/yyyy h:mm AM/PM").unwrap(),
            "5/15/2023 3:30 PM"
        );
        assert_eq!(
            test_format(excel_datetime, "dddd, mmmm dd, yyyy h:mm:ss").unwrap(),
            "Monday, May 15, 2023 15:30:45"
        );
    }

    #[test]
    fn test_fraction_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Fraction formats
        assert_eq!(test_format(1.25, "# ?/?").unwrap(), "1 1/4");
        assert_eq!(test_format(1.33333, "# ?/3").unwrap(), "1 1/3");
        assert_eq!(test_format(1.66667, "# ?/3").unwrap(), "1 2/3");
        assert_eq!(test_format(0.125, "# ?/8").unwrap(), "1/8");
        assert_eq!(test_format(2.5, "# ?/2").unwrap(), "2 1/2");
        assert_eq!(test_format(0.075, "??/???").unwrap(), "3/40");
    }

    #[test]
    fn test_conditional_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Conditional formats
        assert_eq!(
            test_format(10, "[>5]\"High\";[<3]\"Low\";\"Medium\"").unwrap(),
            "High"
        );
        assert_eq!(
            test_format(2, "[>5]\"High\";[<3]\"Low\";\"Medium\"").unwrap(),
            "Low"
        );
        assert_eq!(
            test_format(4, "[>5]\"High\";[<3]\"Low\";\"Medium\"").unwrap(),
            "Medium"
        );

        assert_eq!(
            test_format(
                100,
                "[>=100]\"满分\";[>=90]\"优秀\";[>=60]\"及格\";\"不及格\""
            )
            .unwrap(),
            "满分"
        );
        assert_eq!(
            test_format(
                95,
                "[>=100]\"满分\";[>=90]\"优秀\";[>=60]\"及格\";\"不及格\""
            )
            .unwrap(),
            "优秀"
        );
        assert_eq!(
            test_format(
                70,
                "[>=100]\"满分\";[>=90]\"优秀\";[>=60]\"及格\";\"不及格\""
            )
            .unwrap(),
            "及格"
        );
        assert_eq!(
            test_format(
                50,
                "[>=100]\"满分\";[>=90]\"优秀\";[>=60]\"及格\";\"不及格\""
            )
            .unwrap(),
            "不及格"
        );
    }

    #[test]
    fn test_color_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Color formats
        assert_eq!(test_format(123.45, "[Red]#,##0.00").unwrap(), "[Red]123.45");
        assert_eq!(
            test_format(123.45, "[Blue]#,##0.00").unwrap(),
            "[Blue]123.45"
        );
        assert_eq!(
            test_format(-123.45, "[Red]-#,##0.00;[Blue]#,##0.00").unwrap(),
            "[Red]-123.45"
        );
        assert_eq!(
            test_format(123.45, "[Red]-#,##0.00;[Blue]#,##0.00").unwrap(),
            "[Blue]123.45"
        );
    }

    #[test]
    fn test_text_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Text formats
        assert_eq!(test_format("Hello", "@").unwrap(), "Hello");
        assert_eq!(test_format("Hello", "@ World").unwrap(), "Hello World");
        assert_eq!(
            test_format("Hello", "\"Greeting: \" @").unwrap(),
            "Greeting: Hello"
        );
        assert_eq!(test_format("Hello", "@@@").unwrap(), "HelloHelloHello");
        assert_eq!(test_format("Hello", "*-@").unwrap(), "-----Hello");
    }

    #[test]
    fn test_special_formats() {
        // TODO: LLM Generated, haven't been reviewed.
        // Special formats
        assert_eq!(test_format(1234.567, "General").unwrap(), "1234.567");
        assert_eq!(test_format("Hello", "General").unwrap(), "Hello");
        assert_eq!(test_format(0, "General").unwrap(), "0");

        // Text with escaping
        assert_eq!(test_format("Hello", "\\@").unwrap(), "@");
        assert_eq!(test_format("Hello", "\\\\").unwrap(), "\\");

        // Special numeric formats
        assert_eq!(
            test_format(8001234567.89, "[<=9999999]###-####;(###) ###-####").unwrap(),
            "(800) 123-4567"
        );
        assert_eq!(
            test_format(1234567.89, "[<=9999999]###-####;(###) ###-####").unwrap(),
            "123-4567"
        );
    }

    #[test]
    fn test_with_locale_config() {
        // TODO: LLM Generated, haven't been reviewed.
        // Create a locale config with different separators
        let locale = LocaleConfig {
            decimal_separator: Some(','),
            thousands_separator: Some('.'),
            date_locale: Some("zh-CN".to_string()),
            currency_symbol: Some("￥".to_string()),
        };

        // Format with custom locale
        let format = NumfmtParser::new("#.##0,00")
            .parse()
            .expect("Failed to parse format");

        let result = format_with_parsed(1234.567, &format, Some(locale.clone()))
            .expect("Failed to format value");

        assert_eq!(result, "1.234,57");

        // Another test with currency symbol
        let format = NumfmtParser::new("\"$\"#,##0.00")
            .parse()
            .expect("Failed to parse format");

        let result =
            format_with_parsed(1234.567, &format, Some(locale)).expect("Failed to format value");

        // The currency symbol in the format string should be preserved
        assert_eq!(result, "$1.234,57");
    }
}
