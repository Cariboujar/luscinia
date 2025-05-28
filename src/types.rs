use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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

impl Display for NumFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl NumFormat {
    pub fn pretty(&self) -> String {
        match self {
            NumFormat::ConditionalGeneral(colored) => {
                let (condition, general) = &colored.inner;
                let mut result = String::new();
                result.push_str("ConditionalGeneral:\n");

                if let Some(color) = &colored.color {
                    result.push_str(&format!("  color: {:?}\n", color));
                }
                result.push_str(&format!("  condition: {:?}\n", condition));
                result.push_str(&format!("  general: {:?}\n", general));

                result
            }
            NumFormat::AnyNoCond(any_no_cond) => {
                let mut result = String::new();
                result.push_str("AnyNoCond:\n");

                if let Some(color) = &any_no_cond.color {
                    result.push_str(&format!("  color: {:?}\n", color));
                }

                if let Some(locale) = &any_no_cond.locale {
                    result.push_str(&format!("  locale: {:?}\n", locale));
                }

                match &any_no_cond.inner {
                    TextOr::Text(text) => {
                        result.push_str("  content: Text\n");
                        Self::pretty_text(text, "    ", &mut result);
                    }
                    TextOr::Other(num_or_frac) => {
                        result.push_str("  content: Other\n");
                        Self::pretty_number_or_frac_or_dt(num_or_frac, "    ", &mut result);
                    }
                }

                result
            }
            NumFormat::TwoParts(part1, part2) => {
                let mut result = String::new();
                result.push_str("TwoParts:\n");

                result.push_str("\n  Part 1:\n");
                if let Some(color) = &part1.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part1.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_any_inner(&part1.inner, "      ", &mut result);

                result.push_str("\n  Part 2:\n");
                match part2 {
                    Any::Text(text) => {
                        if let Some(color) = &text.color {
                            result.push_str(&format!("    color: {:?}\n", color));
                        }
                        if let Some(locale) = &text.locale {
                            result.push_str(&format!("    locale: {:?}\n", locale));
                        }
                        result.push_str("    content: Text\n");
                        Self::pretty_text(&text.inner, "      ", &mut result);
                    }
                    Any::Other(other) => {
                        if let Some(color) = &other.color {
                            result.push_str(&format!("    color: {:?}\n", color));
                        }
                        if let Some(locale) = &other.locale {
                            result.push_str(&format!("    locale: {:?}\n", locale));
                        }
                        result.push_str("    content: \n");
                        Self::pretty_any_inner(&other.inner, "      ", &mut result);
                    }
                }

                result
            }
            NumFormat::ThreeParts(part1, part2, part3) => {
                let mut result = String::new();
                result.push_str("ThreeParts:\n");

                result.push_str("\n  Part 1:\n");
                if let Some(color) = &part1.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part1.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_any_inner(&part1.inner, "      ", &mut result);

                result.push_str("\n  Part 2:\n");
                if let Some(color) = &part2.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part2.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_any_inner(&part2.inner, "      ", &mut result);

                result.push_str("\n  Part 3:\n");
                if let Some(color) = &part3.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part3.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }

                match &part3.inner {
                    TextOr::Text(text) => {
                        result.push_str("    content: Text\n");
                        Self::pretty_text(text, "      ", &mut result);
                    }
                    TextOr::Other(num_or_frac) => {
                        result.push_str("    content: Other\n");
                        Self::pretty_number_or_frac_or_dt(num_or_frac, "      ", &mut result);
                    }
                }

                result
            }
            NumFormat::FourParts(part1, part2, part3, part4) => {
                let mut result = String::new();
                result.push_str("FourParts:\n");

                result.push_str("\n  Part 1:\n");
                if let Some(color) = &part1.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part1.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_any_inner(&part1.inner, "      ", &mut result);

                result.push_str("\n  Part 2:\n");
                if let Some(color) = &part2.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part2.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_any_inner(&part2.inner, "      ", &mut result);

                result.push_str("\n  Part 3:\n");
                if let Some(color) = &part3.color {
                    result.push_str(&format!("    color: {:?}\n", color));
                }
                if let Some(locale) = &part3.locale {
                    result.push_str(&format!("    locale: {:?}\n", locale));
                }
                result.push_str("    content: \n");
                Self::pretty_number_or_frac_or_dt(&part3.inner, "      ", &mut result);

                if let Some(text_or_general) = part4 {
                    result.push_str("\n  Part 4:\n");
                    match text_or_general {
                        TextOr::Text(text) => {
                            result.push_str("    content: Text\n");
                            Self::pretty_text(text, "      ", &mut result);
                        }
                        TextOr::Other(general) => {
                            result.push_str("    content: Other\n");
                            result.push_str(&format!("      {:?}", general));
                        }
                    }
                }

                result
            }
        }
    }

    fn pretty_any_inner(inner: &AnyInner, indent: &str, result: &mut String) {
        match inner {
            AnyInner::Data(data) => {
                result.push_str(&format!("{indent}Data:\n"));
                Self::pretty_number_or_frac_or_dt(data, &format!("{}  ", indent), result);
            }
            AnyInner::ConditionalData(cond, data) => {
                result.push_str(&format!("{indent}ConditionalData:\n"));
                if let Some(condition) = cond {
                    result.push_str(&format!("{indent}  condition: {:?}\n", condition));
                }
                result.push_str(&format!("{indent}  data:\n"));
                Self::pretty_number_or_frac_or_dt(data, &format!("{}    ", indent), result);
            }
        }
    }

    fn pretty_number_or_frac_or_dt(data: &NumberOrFracOrDt, indent: &str, result: &mut String) {
        match data {
            NumberOrFracOrDt::Number(number) => {
                result.push_str(&format!("{indent}Number: \n"));
                Self::pretty_number(number, &format!("{}  ", indent), result);
            }
            NumberOrFracOrDt::ParenthesizedNumber(number) => {
                result.push_str(&format!("{indent}ParenthesizedNumber: \n"));
                Self::pretty_number(number, &format!("{}  ", indent), result);
            }
            NumberOrFracOrDt::Fraction(fraction) => {
                result.push_str(&format!("{indent}Fraction: \n"));
                Self::pretty_fraction(fraction, &format!("{}  ", indent), result);
            }
            NumberOrFracOrDt::Datetime(datetime) => {
                result.push_str(&format!("{indent}Datetime: \n"));
                Self::pretty_datetime_tuple(datetime, &format!("{}  ", indent), result);
            }
        }
    }

    fn pretty_number(number: &NFNumber, indent: &str, result: &mut String) {
        // Display num_part
        result.push_str(&format!("{indent}num_part: [\n"));
        for (i, part) in number.num_part.iter().enumerate() {
            Self::pretty_digit_pos_or_other(part, &format!("{indent}  "), result);
            if i < number.num_part.len() - 1 {
                result.push_str(",\n");
            } else {
                result.push('\n');
            }
        }
        result.push_str(&format!("{indent}]\n"));

        // Display exp_part if present
        if let Some((sign, exp)) = &number.exp_part {
            result.push_str(&format!("{indent}exp_part: \n"));
            result.push_str(&format!("{indent}  sign: {:?}\n", sign));
            result.push_str(&format!("{indent}  exp: [\n"));
            for (i, part) in exp.iter().enumerate() {
                Self::pretty_digit_pos_or_other(part, &format!("{indent}    "), result);
                if i < exp.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            }
            result.push_str(&format!("{indent}  ]\n"));
        }

        // Display has_percent
        result.push_str(&format!("{indent}has_percent: {}", number.has_percent));
    }

    fn pretty_digit_pos_or_other<T: std::fmt::Debug>(
        part: &DigitPosOrOther<T>,
        indent: &str,
        result: &mut String,
    ) {
        match part {
            DigitPosOrOther::Digit(digit_pos) => match digit_pos {
                DigitPos::Digit(placeholder) => {
                    let placeholder_str = match placeholder {
                        NumPlaceholder::Zero => "0 (Zero)",
                        NumPlaceholder::Lazy => "# (Lazy)",
                        NumPlaceholder::Space => "? (Space)",
                    };
                    result.push_str(&format!("{indent}Digit: {}", placeholder_str));
                }
                DigitPos::Separator(separator) => {
                    let separator_str = match separator {
                        NumSeparator::Decimal => ". (Decimal)",
                        NumSeparator::NumberGroup => ", (NumberGroup)",
                    };
                    result.push_str(&format!("{indent}Separator: {}", separator_str));
                }
            },
            DigitPosOrOther::Other(other) => {
                result.push_str(&format!("{indent}Other: {:?}", other));
            }
            DigitPosOrOther::LiteralCharSpace(ch) => {
                result.push_str(&format!("{indent}LiteralCharSpace: '{}'", ch));
            }
            DigitPosOrOther::LiteralString(s) => {
                result.push_str(&format!("{indent}LiteralString: \"{}\"", s));
            }
            DigitPosOrOther::FillChar(ch) => {
                result.push_str(&format!("{indent}FillChar: '{}'", ch));
            }
            DigitPosOrOther::EscapedChar(ch) => {
                result.push_str(&format!("{indent}EscapedChar: '{}'", ch));
            }
        }
    }

    fn pretty_fraction(fraction: &NFFraction, indent: &str, result: &mut String) {
        // Display numerator
        result.push_str(&format!("{indent}numerator: [\n"));
        for (i, token) in fraction.numerator.iter().enumerate() {
            Self::pretty_frac_token(token, &format!("{indent}  "), result);
            if i < fraction.numerator.len() - 1 {
                result.push_str(",\n");
            } else {
                result.push('\n');
            }
        }
        result.push_str(&format!("{indent}]\n"));

        // Display denominator
        result.push_str(&format!("{indent}denominator: [\n"));
        for (i, token) in fraction.denominator.iter().enumerate() {
            Self::pretty_frac_token(token, &format!("{indent}  "), result);
            if i < fraction.denominator.len() - 1 {
                result.push_str(",\n");
            } else {
                result.push('\n');
            }
        }
        result.push_str(&format!("{indent}]\n"));

        // Display integer_part if present
        if let Some(int_part) = &fraction.integer_part {
            result.push_str(&format!("{indent}integer_part: [\n"));
            for (i, part) in int_part.iter().enumerate() {
                Self::pretty_digit_pos_or_other(part, &format!("{indent}  "), result);
                if i < int_part.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            }
            result.push_str(&format!("{indent}]\n"));
        }

        // Display ampm_part
        if !fraction.ampm_part.is_empty() {
            result.push_str(&format!("{indent}ampm_part: [\n"));
            for (i, ampm) in fraction.ampm_part.iter().enumerate() {
                let ampm_str = match ampm {
                    AmPm::Full => "Full (AM/PM)",
                    AmPm::Simple => "Simple (A/P)",
                };
                result.push_str(&format!("{indent}  {}", ampm_str));
                if i < fraction.ampm_part.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            }
            result.push_str(&format!("{indent}]\n"));
        }
    }

    fn pretty_frac_token(token: &FracToken, indent: &str, result: &mut String) {
        match token {
            FracToken::Placeholder(placeholder) => {
                let placeholder_str = match placeholder {
                    NumPlaceholder::Zero => "0 (Zero)",
                    NumPlaceholder::Lazy => "# (Lazy)",
                    NumPlaceholder::Space => "? (Space)",
                };
                result.push_str(&format!("{indent}Placeholder: {}", placeholder_str));
            }
            FracToken::Percent => {
                result.push_str(&format!("{indent}Percent"));
            }
        }
    }

    fn pretty_datetime_tuple(dt: &DatetimeTuple, indent: &str, result: &mut String) {
        // Display datetime part if present
        if let Some(datetime) = &dt.0 {
            result.push_str(&format!("{indent}datetime: \n"));
            Self::pretty_datetime(datetime, &format!("{}  ", indent), result);
        }

        // Display general part if present
        if let Some(general) = &dt.1 {
            result.push_str(&format!("{indent}general: {:?}\n", general));
        }

        // Display second datetime part if present
        if let Some(datetime2) = &dt.2 {
            result.push_str(&format!("{indent}datetime2: \n"));
            Self::pretty_datetime(datetime2, &format!("{}  ", indent), result);
        }
    }

    fn pretty_datetime(dt: &NFDatetime, indent: &str, result: &mut String) {
        result.push_str(&format!("{indent}components: [\n"));
        if !dt.components.is_empty() {
            for (i, component) in dt.components.iter().enumerate() {
                Self::pretty_datetime_component(component, &format!("{}  ", indent), result);
                if i < dt.components.len() - 1 {
                    result.push_str(",\n");
                } else {
                    result.push('\n');
                }
            }
        }
        result.push_str(&format!("{indent}]"));
    }

    fn pretty_datetime_component(
        component: &NFDatetimeComponent,
        indent: &str,
        result: &mut String,
    ) {
        match component {
            NFDatetimeComponent::Token(token) => match token {
                NFDateTimeToken::EraG(format) => {
                    let format_str = match format {
                        EraFormatG::OneDigit => "OneDigit",
                        EraFormatG::TwoDigit => "TwoDigit",
                        EraFormatG::ThreeDigit => "ThreeDigit",
                    };
                    result.push_str(&format!("{indent}Token: Era({})", format_str));
                }
                NFDateTimeToken::CalendarB(calendar) => {
                    let calendar_str = match calendar {
                        CalendarTypeB::Gregorian => "Gregorian",
                        CalendarTypeB::Hijri => "Hijri",
                    };
                    result.push_str(&format!("{indent}Token: Calendar({})", calendar_str));
                }
                NFDateTimeToken::Year(format) => {
                    result.push_str(&format!("{indent}Token: Year({:?})", format));
                }
                NFDateTimeToken::EraYear(format) => {
                    result.push_str(&format!("{indent}Token: EraYear({:?})", format));
                }
                NFDateTimeToken::Month(format) => {
                    let format_str = format!("{:?}", format);
                    let month_desc = match format {
                        MonthFormat(1) => "1 (Single digit)",
                        MonthFormat(2) => "2 (Two digits)",
                        MonthFormat(3) => "3 (Three-letter abbrev)",
                        MonthFormat(4) => "4 (Full name)",
                        MonthFormat(5) => "5 (First letter)",
                        _ => format_str.as_str(),
                    };
                    result.push_str(&format!("{indent}Token: Month({})", month_desc));
                }
                NFDateTimeToken::Day(format) => {
                    let format_str = format!("{:?}", format);
                    let day_desc = match format {
                        DayFormat(1) => "1 (Single digit)",
                        DayFormat(2) => "2 (Two digits)",
                        DayFormat(3) => "3 (Three-letter abbrev)",
                        DayFormat(4) => "4 (Full name)",
                        _ => format_str.as_str(),
                    };
                    result.push_str(&format!("{indent}Token: Day({})", day_desc));
                }
                NFDateTimeToken::Hour(format) => {
                    result.push_str(&format!("{indent}Token: Hour({:?})", format));
                }
                NFDateTimeToken::Minute(format) => {
                    result.push_str(&format!("{indent}Token: Minute({:?})", format));
                }
                NFDateTimeToken::Second(format) => {
                    result.push_str(&format!("{indent}Token: Second({:?})", format));
                }
                NFDateTimeToken::Abs(abs_token) => {
                    result.push_str(&format!("{indent}Token: Abs({:?})", abs_token));
                }
            },
            NFDatetimeComponent::SubSecond(format) => {
                result.push_str(&format!("{indent}SubSecond: {:?}", format));
            }
            NFDatetimeComponent::DateSeparator => {
                result.push_str(&format!("{indent}DateSeparator"));
            }
            NFDatetimeComponent::TimeSeparator => {
                result.push_str(&format!("{indent}TimeSeparator"));
            }
            NFDatetimeComponent::AMPM(ampm) => {
                let ampm_str = match ampm {
                    AmPm::Full => "Full (AM/PM)",
                    AmPm::Simple => "Simple (A/P)",
                };
                result.push_str(&format!("{indent}AMPM: {}", ampm_str));
            }
            NFDatetimeComponent::Literal(text) => {
                result.push_str(&format!("{indent}Literal: \"{}\"", text));
            }
        }
    }

    fn pretty_text(text: &NFText, indent: &str, result: &mut String) {
        result.push_str(&format!("{indent}elements: ["));
        if !text.elements.is_empty() {
            result.push('\n');
            for element in &text.elements {
                result.push_str(&format!("{indent}  {:?}\n", element));
            }
            result.push_str(indent);
        }
        result.push(']');
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyInner {
    Data(NumberOrFracOrDt),
    ConditionalData(Option<NFPartCondition>, NumberOrFracOrDt),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Any {
    Text(SectionWrapper<NFText>),
    Other(SectionWrapper<AnyInner>),
}

pub type AnyNoText = SectionWrapper<AnyInner>;
pub type AnyNoCond = SectionWrapper<TextOr<NumberOrFracOrDt>>;
pub type AnyNoTextNoCond = SectionWrapper<NumberOrFracOrDt>;

/// [NFDateTime] [NFGeneral] [NFDateTime]
#[derive(Debug, Clone, PartialEq)]
pub struct DatetimeTuple(
    pub Option<NFDatetime>,
    pub Option<NFGeneral>,
    pub Option<NFDatetime>,
);

#[derive(Debug, Clone, PartialEq)]
pub struct NFDatetime {
    pub components: Vec<NFDatetimeComponent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NFDatetimeComponent {
    Token(NFDateTimeToken),
    SubSecond(SubSecondFormat),
    DateSeparator,
    TimeSeparator,
    AMPM(AmPm),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextOr<T> {
    Text(NFText),
    Other(T),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SectionWrapper<T> {
    pub is_thai_prefixed: bool,
    pub locale: Option<PartLocaleID>,
    pub color: Option<NFPartColor>,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaybeConditional<T> {
    pub condition: Option<NFPartCondition>,
    pub inner: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberOrFracOrDt {
    Number(NFNumber),
    ParenthesizedNumber(NFNumber),
    Fraction(NFFraction),
    Datetime(DatetimeTuple),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFGeneral {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFNumber {
    pub num_part: Vec<DigitPosOrOther<Percent>>,
    pub exp_part: Option<(Sign, Vec<DigitPosOrOther<Percent>>)>,
    pub has_percent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFFraction {
    pub numerator: Vec<FracToken>,
    pub denominator: Vec<FracToken>,
    pub integer_part: Option<Vec<DigitPosOrOther<Percent>>>,
    pub ampm_part: Vec<AmPm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FracToken {
    Placeholder(NumPlaceholder),
    Percent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextFormatElement {
    AtPlaceholder,
    AmPm(AmPm),
    LiteralString(String),
    LiteralCharSpace(char),
    FillChar(char),
    EscapedChar(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NFText {
    pub elements: Vec<TextFormatElement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AmPm {
    Full,
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
/// NFPartNumToken1
pub enum NumPlaceholder {
    /// 0
    Zero,
    /// #
    Lazy,
    /// ?
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NumSeparator {
    /// .
    Decimal,
    /// ,
    NumberGroup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// NFPartNumToken2
pub enum DigitPos {
    Digit(NumPlaceholder),
    Separator(NumSeparator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigitPosOrOther<T> {
    Digit(DigitPos),
    Other(T),
    LiteralCharSpace(char),
    LiteralString(String),
    FillChar(char),
    EscapedChar(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percent {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NFPartColor {
    Intl(DefinedColor),
    Color(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum YearFormat {
    TwoDigit,
    FourDigit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraYearFormat {
    Short,
    Long,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraFormatG {
    OneDigit,
    TwoDigit,
    ThreeDigit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarTypeB {
    Gregorian,
    Hijri,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 1-5
pub struct MonthFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 1-4
pub struct DayFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HourFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsHourFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MinuteFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsMinuteFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecondFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsSecondFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubSecondFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCondOperator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// [>=1.0]
#[derive(Debug, Clone, PartialEq)]
pub struct NFPartCondition {
    pub op: NFCondOperator,
    pub value: f64,
}

impl Eq for NFPartCondition {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsTimeToken {
    AbsHour(AbsHourFormat),
    AbsMinute(AbsMinuteFormat),
    AbsSecond(AbsSecondFormat),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFDateTimeToken {
    EraG(EraFormatG),
    CalendarB(CalendarTypeB),
    Year(YearFormat),
    EraYear(EraYearFormat),
    Month(MonthFormat),
    Day(DayFormat),
    Hour(HourFormat),
    Minute(MinuteFormat),
    Second(SecondFormat),
    Abs(AbsTimeToken),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedLanguageInfo {
    Complex {
        raw_value: u32,
        lid: u16,
        calendar_type_byte: u8,
        number_system_type_byte: u8,
    },
    SystemLongDate,
    SystemTimeFormat,
    RawLID(u16),
}

impl ParsedLanguageInfo {
    pub fn use_specified_calendar(&self) -> Option<bool> {
        match self {
            ParsedLanguageInfo::Complex {
                calendar_type_byte, ..
            } => Some((calendar_type_byte & 0x80) != 0),
            _ => None,
        }
    }

    pub fn calendar_type_value(&self) -> Option<u8> {
        match self {
            ParsedLanguageInfo::Complex {
                calendar_type_byte, ..
            } => Some(calendar_type_byte & 0x7F),
            _ => None,
        }
    }

    pub fn use_specified_number_system(&self) -> Option<bool> {
        match self {
            ParsedLanguageInfo::Complex {
                number_system_type_byte,
                ..
            } => Some((number_system_type_byte & 0x80) != 0),
            _ => None,
        }
    }

    pub fn number_system_type_value(&self) -> Option<u8> {
        match self {
            ParsedLanguageInfo::Complex {
                number_system_type_byte,
                ..
            } => Some(number_system_type_byte & 0x7F),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartLocaleID {
    // "USD"
    pub currency_symbol: String,
    pub language_info: Option<ParsedLanguageInfo>,
}

impl PartLocaleID {
    pub fn from_parsed_peg(
        currency_symbol_chars: Vec<char>,
        hex_digits_opt: Option<Vec<u8>>,
    ) -> Self {
        let lang_info = hex_digits_opt.and_then(|digits| {
            if digits.len() < 3 || digits.len() > 8 {
                return None;
            }
            let mut num_val: u32 = 0;
            for &digit_val in &digits {
                num_val = (num_val << 4) | (digit_val as u32);
            }

            match num_val {
                0xf800 => Some(ParsedLanguageInfo::SystemLongDate),
                0xf400 => Some(ParsedLanguageInfo::SystemTimeFormat),
                _ => {
                    let lid = (num_val & 0xFFFF) as u16;
                    if digits.len() <= 4 {
                        Some(ParsedLanguageInfo::RawLID(lid))
                    } else {
                        let calendar_type_byte = ((num_val >> 16) & 0xFF) as u8;
                        let number_system_type_byte = ((num_val >> 24) & 0xFF) as u8;
                        Some(ParsedLanguageInfo::Complex {
                            raw_value: num_val,
                            lid,
                            calendar_type_byte,
                            number_system_type_byte,
                        })
                    }
                }
            }
        });

        PartLocaleID {
            currency_symbol: currency_symbol_chars.into_iter().collect(),
            language_info: lang_info,
        }
    }
}
