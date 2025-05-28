use std::{collections::BTreeMap, sync::OnceLock};

pub static BUILTIN_FORMATS: OnceLock<BTreeMap<u8, NumFormat>> = OnceLock::new();
pub fn builtin_formats() -> &'static BTreeMap<u8, NumFormat> {
    BUILTIN_FORMATS.get_or_init(|| {
        include_str!("builtin.tsv")
            .lines()
            .map(|line| {
                let mut parts = line.split('\t');
                let id = parts.next().unwrap().parse::<u8>().unwrap();
                let format = NumfmtParser::new(parts.next().unwrap()).parse().unwrap();
                (id, format)
            })
            .collect()
    })
}
pub fn builtin_format(id: u8) -> Option<&'static NumFormat> {
    builtin_formats().get(&id)
}

pub type PResult<T> = Result<T, peg::error::ParseError<peg::str::LineCol>>;

#[derive(Debug, PartialEq, Eq)]
pub enum NumFormat {
    ConditionalGeneral(MaybeColored<(NFPartCondition, NFGeneral)>),
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

type Any = MaybeColored<TextOr<MaybeConditional<NumberOrFracOrDt>>>;
type AnyNoText = MaybeColored<MaybeConditional<NumberOrFracOrDt>>;
type AnyNoCond = MaybeColored<TextOr<NumberOrFracOrDt>>;
type AnyNoTextNoCond = MaybeColored<NumberOrFracOrDt>;

/// [NFDateTime] [NFGeneral] [NFDateTime]
#[derive(Debug, PartialEq, Eq)]
pub struct DatetimeTuple(
    pub Option<NFDatetime>,
    pub Option<NFGeneral>,
    pub Option<NFDatetime>,
);

/// TODO
#[derive(Debug, PartialEq, Eq)]
pub struct NFDatetime {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFDatetimeComponent {
    Token(NFDateTimeToken),
    SubSecond(SubSecondFormat),
    DateSeparator,
    TimeSeparator,
    AMPM(AmPm),
}

#[derive(Debug, PartialEq, Eq)]
pub enum TextOr<T> {
    Text(NFText),
    Other(T),
}

#[derive(Debug, PartialEq, Eq)]
pub struct MaybeColored<T> {
    pub color: Option<NFPartColor>,
    pub inner: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MaybeConditional<T> {
    pub condition: Option<NFPartCondition>,
    pub inner: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NumberOrFracOrDt {
    Number(NFNumber),
    Fraction(NFFraction),
    Datetime(DatetimeTuple),
}

/// TODO
#[derive(Debug, PartialEq, Eq)]
pub struct NFGeneral {}

/// TODO
#[derive(Debug, PartialEq, Eq)]
pub struct NFNumber {}

/// TODO
#[derive(Debug, PartialEq, Eq)]
pub struct NFFraction {}

/// true => @
/// false => INTL-AMPM
#[derive(Debug, PartialEq, Eq)]
pub struct NFText {
    format: Vec<bool>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[repr(u8)]
pub enum SubSecondFormat {
    OneChar,
    TwoChar,
    ThreeChar,
}

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
#[derive(Debug)]
pub struct NFPartCondition {
    pub op: NFCondOperator,
    pub value: f64,
}

impl PartialEq for NFPartCondition {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.value.to_bits() == other.value.to_bits()
    }
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
    Year(YearFormat),
    Month(MonthFormat),
    Day(DayFormat),
    Hour(HourFormat),
    Minute(MinuteFormat),
    Second(SecondFormat),
    Abs(AbsTimeToken),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartLocaleID {
    name: String,
    suffix: Option<Vec<u8>>,
}

// The comment "Line xx" refers to the line number
// in the original ABNF specification.
//
// [MS-OE376] 2.1.739 Part 4 Section 3.8.30, numFmt (Number Format)
//
// https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/0e59abdb-7f4e-48fc-9b89-67832fa11789
peg::parser! {
    grammar numfmt_parser() for str {
        pub rule all() -> NumFormat // Line 1
            = f1:nf_any_no_text() ascii_semicolon() f2:nf_any_no_text() ascii_semicolon() f3:nf_any_no_text_no_cond() ascii_semicolon() f4:all_f4()? {
                NumFormat::FourParts(
                    f1,
                    f2,
                    f3,
                    f4,
                )
            }
            / f1:nf_any_no_text() ascii_semicolon() f2:nf_any_no_text() ascii_semicolon() f3:nf_any_no_cond() {
                NumFormat::ThreeParts(
                    f1,
                    f2,
                    f3,
                )
            }
            / f1:nf_any_no_text() ascii_semicolon() f2:nf_any() {
                NumFormat::TwoParts(
                    f1,
                    f2,
                )
            }
            / f:nf_any_no_cond() { NumFormat::AnyNoCond(f) }
            / color:nf_part_color()? cond:nf_part_cond() g:nf_general() {
                NumFormat::ConditionalGeneral(MaybeColored {
                    color,
                    inner: (cond, g),
                })
            }

            rule all_f4() -> TextOr<NFGeneral> // Custom
                = t:nf_text() { TextOr::Text(t) }
                / f:nf_general() { TextOr::Other(f) }

            rule num_or_frac_or_dt() -> NumberOrFracOrDt // Custom
                = n:nf_number() { NumberOrFracOrDt::Number(n) }
                // TODO: Fraction
                // / f:nf_fraction() { NumberOrFracOrDt::Fraction(f) }
                / dt:datetime_tuple() { NumberOrFracOrDt::Datetime(dt) }

            rule datetime_tuple() -> DatetimeTuple // Custom
                = dt1:nf_datetime()? g:nf_general()? dt2:nf_datetime()? {
                    DatetimeTuple(dt1, g, dt2)
                }

        rule nf_any() -> Any // Line 2
            = color:nf_part_color()? text:nf_text() {
                MaybeColored {
                    color,
                    inner: TextOr::Text(text),
                }
            }
            / color:nf_part_color()? condition:nf_part_cond()? data:num_or_frac_or_dt() {
                MaybeColored {
                    color,
                    inner: TextOr::Other(MaybeConditional {
                        condition,
                        inner: data,
                    }),
                }
            }

        rule nf_any_no_text() -> AnyNoText // Line 3
            = color:nf_part_color()? condition:nf_part_cond()? data:num_or_frac_or_dt() {
                MaybeColored {
                    color,
                    inner: MaybeConditional {
                        condition,
                        inner: data,
                    },
                }
            }

        rule nf_any_no_cond() -> AnyNoCond // Line 4
            = color:nf_part_color()? text:nf_text() {
                MaybeColored {
                    color,
                    inner: TextOr::Text(text),
                }
            }
            / color:nf_part_color()? data:num_or_frac_or_dt() {
                MaybeColored {
                    color,
                    inner: TextOr::Other(data),
                }
            }

        rule nf_any_no_text_no_cond() -> AnyNoTextNoCond // Line 5
            = color:nf_part_color()? data:num_or_frac_or_dt() {
                MaybeColored {
                    color,
                    inner: data,
                }
            }

        rule nf_general() -> NFGeneral // Line 6
            = intl_numfmt_general() { NFGeneral {} }

        rule nf_number() -> NFNumber // Line 7
            = nf_part_num() { NFNumber {} } // TODO: Line 13, behavior & validation

        rule nf_datatime_token() -> NFDateTimeToken // Line 8
            = y:nf_part_year() { NFDateTimeToken::Year(y) }
            / m:nf_part_month() { NFDateTimeToken::Month(m) }
            / d:nf_part_day() { NFDateTimeToken::Day(d) }
            / h:nf_part_hour() { NFDateTimeToken::Hour(h) }
            / m:nf_part_minute() { NFDateTimeToken::Minute(m) }
            / s:nf_part_second() { NFDateTimeToken::Second(s) }
            / a:nf_abs_time_token() { NFDateTimeToken::Abs(a) }

        rule nf_abs_time_token() -> AbsTimeToken // Line 9
            = h:nf_part_abs_hour() { AbsTimeToken::AbsHour(h) }
            / m:nf_part_abs_minute() { AbsTimeToken::AbsMinute(m) }
            / s:nf_part_abs_second() { AbsTimeToken::AbsSecond(s) }

        rule nf_datetime() -> NFDatetime // Line 10
            = ampms:intl_ampm()* dt_tokens:nf_datatime_token()+ components:nf_datetime_component()* {
                NFDatetime {} // TODO: implement
            }

            rule nf_datetime_component() -> NFDatetimeComponent // Custom
                = t:nf_datatime_token() { NFDatetimeComponent::Token(t) }
                / s:nf_part_sub_second() { NFDatetimeComponent::SubSecond(s) }
                / intl_char_date_sep() { NFDatetimeComponent::DateSeparator }
                / intl_char_time_sep() { NFDatetimeComponent::TimeSeparator }
                / ampm:intl_ampm() { NFDatetimeComponent::AMPM(ampm) }

        rule nf_text() -> NFText // Line 11
            = f:(ascii_commercial_at()+) s:(nf_text_is_at()*) {
                NFText { format: f.iter().map(|_| true).chain(s.iter().copied()).collect() }
            }
            / f:(nf_text_is_at()*) s:(ascii_commercial_at()+) {
                NFText { format: f.iter().copied().chain(s.iter().map(|_| true)).collect() }
            }

            rule nf_text_is_at() -> bool
                = ascii_commercial_at() { true }
                / intl_ampm() { false }

        rule nf_fraction() -> () // Line 12
            = {} // TODO

        rule nf_part_num() -> Vec<DigitPosOrOther<Percent>> // Line 13
            = tks:nf_part_num_tk2_or_percent()+ {
                ?
                if tks.first().is_some_and(|t| matches!(t, DigitPosOrOther::Other(_)))
                    && tks.last().is_some_and(|t| matches!(t, DigitPosOrOther::Other(_)))
                {
                    Err("Invalid number format: percent sign at both ends")
                } else {
                    Ok(tks)
                }
            }

            rule nf_part_num_tk2_or_percent() -> DigitPosOrOther<Percent> // Custom
                = t:nf_part_num_token2() { DigitPosOrOther::Digit(t) }
                / ascii_percent_sign() { DigitPosOrOther::Other(Percent {}) }

        rule nf_part_exponential() -> Sign // Line 14
            = ascii_capital_letter_e() sgn:nf_part_sign() { sgn }

        rule nf_part_year() -> YearFormat // Line 15
            = "yyyy" { YearFormat::FourDigit }
            / "yy" { YearFormat::TwoDigit }

        rule nf_part_month() -> MonthFormat // Line 16
            = m:(ascii_small_letter_m()*<1,5>) {
                MonthFormat(m.len() as u8)
            }

        rule nf_part_day() -> DayFormat // Line 17
            = d:(ascii_small_letter_d()*<1,4>) {
                DayFormat(d.len() as u8)
            }

        rule nf_part_hour() -> HourFormat // Line 18
            = "hh" { HourFormat::TwoChar }
            / "h" { HourFormat::OneChar }

        rule nf_part_abs_hour() -> AbsHourFormat // Line 19
            = ascii_left_square_bracket() h:ascii_small_letter_h()+ ascii_right_square_bracket() {
                AbsHourFormat(h.len() as u8)
            }

        rule nf_part_minute() -> MinuteFormat // Line 20
            = "mm" { MinuteFormat::TwoChar }
            / "m" { MinuteFormat::OneChar }

        rule nf_part_abs_minute() -> AbsMinuteFormat // Line 21
            = ascii_left_square_bracket() m:ascii_small_letter_m()+ ascii_right_square_bracket() {
                AbsMinuteFormat(m.len() as u8)
            }

        rule nf_part_second() -> SecondFormat // Line 22
            = "ss" { SecondFormat::TwoChar }
            / "s" { SecondFormat::OneChar }

        rule nf_part_abs_second() -> AbsSecondFormat // Line 23
            = ascii_left_square_bracket() s:ascii_small_letter_s()+ ascii_right_square_bracket() {
                AbsSecondFormat(s.len() as u8)
            }

        rule nf_part_sub_second() -> SubSecondFormat // Line 24
            = "sss" { SubSecondFormat::ThreeChar }
            / "ss" { SubSecondFormat::TwoChar }
            / "s" { SubSecondFormat::OneChar }

        rule nf_part_cond() -> NFPartCondition // Line 25
            = ascii_left_square_bracket() op:nf_part_comp_oper() value:nf_part_cond_num() ascii_right_square_bracket() {
                NFPartCondition { op, value }
            }

        rule nf_part_comp_oper() -> NFCondOperator // Line 26
            = ascii_equals_sign() { NFCondOperator::Equal }
            / ascii_greater_than_sign() ascii_equals_sign() {
                NFCondOperator::GreaterThanOrEqual
            }
            / ascii_greater_than_sign() { NFCondOperator::GreaterThan }
            / ascii_less_than_sign() ascii_equals_sign() {
                NFCondOperator::LessThanOrEqual
            }
            / ascii_less_than_sign() { NFCondOperator::LessThan }

        rule nf_part_locale_id() -> PartLocaleID // Line 27
            = ascii_left_square_bracket() ascii_dollar_sign() name:utf16_any()+ suffix:nf_part_locale_id_suffix()? ascii_right_square_bracket() {
                PartLocaleID {
                    name: name.iter().collect(),
                    suffix,
                }
            }

            rule nf_part_locale_id_suffix() -> Vec<u8>
                = ascii_hyphen_minus() val:ascii_digit_hexadecimal()*<3,8> {
                    val.into_iter().collect()
                }

        rule nf_part_cond_num() -> f64 // Line 28
            = neg:ascii_hyphen_minus()? int_p:nf_part_int_num() dec_p:nf_part_cond_num_dec()? exp:nf_part_cond_num_exp()? {
                let mut value = int_p.iter().fold(0.0, |acc, &x| acc * 10.0 + x as f64);
                if let Some(dec) = dec_p {
                    value += dec.iter().rev().fold(0.0, |acc, &x| acc / 10.0 + x as f64) / 10.0
                }
                if let Some((exp_sign, exp_int)) = exp {
                    value *= 10f64.powi(match exp_sign {
                        Sign::Plus => exp_int.iter().fold(0, |acc, &x| acc * 10 + x) as i32,
                        Sign::Minus => -(exp_int.iter().fold(0, |acc, &x| acc * 10 + x) as i32),
                    })
                }
                if neg.is_some() {
                    -value
                } else {
                    value
                }
            }

            rule nf_part_cond_num_dec() -> Vec<u8>
                = intl_char_decimal_sep() val:nf_part_int_num() { val }

            rule nf_part_cond_num_exp() -> (Sign, Vec<u8>)
                = exp:nf_part_exponential() int_p:nf_part_int_num() { (exp, int_p) }

        rule nf_part_sign() -> Sign // Line 29
            = ascii_plus_sign() { Sign::Plus }
            / ascii_hyphen_minus() { Sign::Minus }

        rule nf_part_color() -> NFPartColor // Line 30
            = ascii_left_square_bracket() c:intl_color() ascii_right_square_bracket() {
                NFPartColor::Intl(c)
            }
            / ascii_left_square_bracket() nf_part_str_color() id:nf_part_1to56() ascii_right_square_bracket() {
                NFPartColor::Color(id)
            }

        rule nf_part_1to56() -> u8 // Line 31
            = ts:nf_part_number_1to4() os:ascii_digit() { ts * 10 + os }
            / ascii_digit_five() os:nf_part_number_1to6() { 50 + os }
            / ascii_digit_five() ascii_digit_zero() { 50 }
            / os:nf_part_number_1to9() { os }

        rule nf_part_int_num() -> Vec<u8> // Line 32
            = ascii_digit()+

        rule nf_part_num_token1() -> NumPlaceholder // Line 33
            = ascii_number_sign() { NumPlaceholder::Lazy }
            / ascii_digit_zero() { NumPlaceholder::Zero }
            / ascii_question_mark() { NumPlaceholder::Space }

        rule nf_part_num_token2() -> DigitPos // Line 34
            = t1:nf_part_num_token1() { DigitPos::Digit(t1) }
            / intl_char_decimal_sep() { DigitPos::Separator(NumSeparator::Decimal) }
            / intl_char_numgrp_sep() { DigitPos::Separator(NumSeparator::NumberGroup) }

        rule nf_part_fraction() -> () // Line 35
            // Guessing that only one percent sign is allowed
            // = f:nf_part_int_num()+ s:nf_part_fraction_num_or_percent()*
            = {} // TODO

            rule nf_part_fraction_num_or_percent() -> Option<Vec<u8>> // Some for num, None for %
                = n:nf_part_int_num() { Some(n) }
                / ascii_percent_sign() { None }

        rule nf_part_number_1to4() -> u8 // Line 36
            = ascii_digit_one() { 1 }
            / ascii_digit_two() { 2 }
            / ascii_digit_three() { 3 }
            / ascii_digit_four() { 4 }

        rule nf_part_number_1to6() -> u8 // Line 37
            = nf_part_number_1to4()
            / ascii_digit_five() { 5 }
            / ascii_digit_six() { 6 }

        rule nf_part_number_1to9() -> u8 // Line 38
            = nf_part_number_1to6()
            / ascii_digit_seven() { 7 }
            / ascii_digit_eight() { 8 }
            / ascii_digit_nine() { 9 }

        rule nf_part_str_color() -> () // Line 39
            = ascii_capital_letter_c() ascii_small_letter_o() ascii_small_letter_l() ascii_small_letter_o() ascii_small_letter_r() {}

        rule literal_char() -> char // Line 40
            = ascii_reverse_solidus() c:utf16_any() { c }

        rule literal_char_repeat() -> char // Line 41
            = ascii_asterisk() c:utf16_any() { c }

        rule literal_string() -> String // Line 42
            = ascii_quotation_mark() s:utf16_any_without_quote()+ ascii_quotation_mark() { s.iter().collect() }
            / s:literal_char()+ { s.iter().collect() }

        rule utf16_any_without_quote() -> char // Line 43
            = ['\u{0000}'..='\u{0021}' | '\u{0023}'..='\u{FFFF}']

        rule literal_char_space() -> char // Line 44
            = ascii_low_line() c:utf16_any() { c }

        rule intl_char_decimal_sep() -> () // Line 45
            = ascii_full_stop()

        rule intl_char_numgrp_sep() -> () // Line 46
            = ascii_comma()

        rule intl_char_date_sep() -> () // Line 47
            = ascii_solidus()

        rule intl_char_time_sep() -> () // Line 48
            = ascii_colon()

        rule intl_color() -> DefinedColor // Line 49
            // TODO: benchmark, use "Black" or ascii_capital_letter_b() ...?
            = "Black" { DefinedColor::Black }
            / "Blue" { DefinedColor::Blue }
            / "Cyan" { DefinedColor::Cyan }
            / "Green" { DefinedColor::Green }
            / "Magenta" { DefinedColor::Magenta }
            / "Red" { DefinedColor::Red }
            / "White" { DefinedColor::White }
            / "Yellow" { DefinedColor::Yellow }

        rule intl_numfmt_general() -> () // Line 50
            = "General" { }

        rule intl_ampm() -> AmPm // Line 51
            = ascii_capital_letter_p() ascii_capital_letter_m() ascii_solidus() ascii_capital_letter_a() ascii_capital_letter_m() { AmPm::Full }
            / "A/P" { AmPm::Simple }

        rule utf16_any() -> char // Line 52
            = c:(['\u{0000}'..='\u{FFFF}']) { c }

        rule ascii_space() -> ()
            = ['\x20'] { }

        rule ascii_exclamation_mark() -> ()
            = ['\x21'] { }

        rule ascii_quotation_mark() -> ()
            = ['\x22'] { }

        rule ascii_number_sign() -> ()
            = ['\x23'] { }

        rule ascii_dollar_sign() -> ()
            = ['\x24'] { }

        rule ascii_percent_sign() -> ()
            = ['\x25'] { }

        rule ascii_ampersand() -> ()
            = ['\x26'] { }

        rule ascii_apostrophe() -> ()
            = ['\x27'] { }

        rule ascii_left_parenthesis() -> ()
            = ['\x28'] { }

        rule ascii_right_parenthesis() -> ()
            = ['\x29'] { }

        rule ascii_asterisk() -> ()
            = ['\x2A'] { }

        rule ascii_plus_sign() -> ()
            = ['\x2B'] { }

        rule ascii_comma() -> ()
            = ['\x2C'] { }

        rule ascii_hyphen_minus() -> ()
            = ['\x2D'] { }

        rule ascii_full_stop() -> ()
            = ['\x2E'] { }

        rule ascii_solidus() -> ()
            = ['\x2F'] { }

        rule ascii_digit_zero() -> ()
            = ['\x30'] { }

        rule ascii_digit_one() -> ()
            = ['\x31'] { }

        rule ascii_digit_two() -> ()
            = ['\x32'] { }

        rule ascii_digit_three() -> ()
            = ['\x33'] { }

        rule ascii_digit_four() -> ()
            = ['\x34'] { }

        rule ascii_digit_five() -> ()
            = ['\x35'] { }

        rule ascii_digit_six() -> ()
            = ['\x36'] { }

        rule ascii_digit_seven() -> ()
            = ['\x37'] { }

        rule ascii_digit_eight() -> ()
            = ['\x38'] { }

        rule ascii_digit_nine() -> ()
            = ['\x39'] { }

        rule ascii_colon() -> ()
            = ['\x3A'] { }

        rule ascii_semicolon() -> ()
            = ['\x3B'] { }

        rule ascii_less_than_sign() -> ()
            = ['\x3C'] { }

        rule ascii_equals_sign() -> ()
            = ['\x3D'] { }

        rule ascii_greater_than_sign() -> ()
            = ['\x3E'] { }

        rule ascii_question_mark() -> ()
            = ['\x3F'] { }

        rule ascii_commercial_at() -> ()
            = ['\x40'] { }

        rule ascii_capital_letter_a() -> ()
            = ['\x41'] { }

        rule ascii_capital_letter_b() -> ()
            = ['\x42'] { }

        rule ascii_capital_letter_c() -> ()
            = ['\x43'] { }

        rule ascii_capital_letter_d() -> ()
            = ['\x44'] { }

        rule ascii_capital_letter_e() -> ()
            = ['\x45'] { }

        rule ascii_capital_letter_f() -> ()
            = ['\x46'] { }

        rule ascii_capital_letter_g() -> ()
            = ['\x47'] { }

        rule ascii_capital_letter_h() -> ()
            = ['\x48'] { }

        rule ascii_capital_letter_i() -> ()
            = ['\x49'] { }

        rule ascii_capital_letter_j() -> ()
            = ['\x4A'] { }

        rule ascii_capital_letter_k() -> ()
            = ['\x4B'] { }

        rule ascii_capital_letter_l() -> ()
            = ['\x4C'] { }

        rule ascii_capital_letter_m() -> ()
            = ['\x4D'] { }

        rule ascii_capital_letter_n() -> ()
            = ['\x4E'] { }

        rule ascii_capital_letter_o() -> ()
            = ['\x4F'] { }

        rule ascii_capital_letter_p() -> ()
            = ['\x50'] { }

        rule ascii_capital_letter_q() -> ()
            = ['\x51'] { }

        rule ascii_capital_letter_r() -> ()
            = ['\x52'] { }

        rule ascii_capital_letter_s() -> ()
            = ['\x53'] { }

        rule ascii_capital_letter_t() -> ()
            = ['\x54'] { }

        rule ascii_capital_letter_u() -> ()
            = ['\x55'] { }

        rule ascii_capital_letter_v() -> ()
            = ['\x56'] { }

        rule ascii_capital_letter_w() -> ()
            = ['\x57'] { }

        rule ascii_capital_letter_x() -> ()
            = ['\x58'] { }

        rule ascii_capital_letter_y() -> ()
            = ['\x59'] { }

        rule ascii_capital_letter_z() -> ()
            = ['\x5A'] { }

        rule ascii_left_square_bracket() -> ()
            = ['\x5B'] { }

        rule ascii_reverse_solidus() -> ()
            = ['\x5C'] { }

        rule ascii_right_square_bracket() -> ()
            = ['\x5D'] { }

        rule ascii_circumflex_accent() -> ()
            = ['\x5E'] { }

        rule ascii_low_line() -> ()
            = ['\x5F'] { }

        rule ascii_grave_accent() -> ()
            = ['\x60'] { }

        rule ascii_small_letter_a() -> ()
            = ['\x61'] { }

        rule ascii_small_letter_b() -> ()
            = ['\x62'] { }

        rule ascii_small_letter_c() -> ()
            = ['\x63'] { }

        rule ascii_small_letter_d() -> ()
            = ['\x64'] { }

        rule ascii_small_letter_e() -> ()
            = ['\x65'] { }

        rule ascii_small_letter_f() -> ()
            = ['\x66'] { }

        rule ascii_small_letter_g() -> ()
            = ['\x67'] { }

        rule ascii_small_letter_h() -> ()
            = ['\x68'] { }

        rule ascii_small_letter_i() -> ()
            = ['\x69'] { }

        rule ascii_small_letter_j() -> ()
            = ['\x6A'] { }

        rule ascii_small_letter_k() -> ()
            = ['\x6B'] { }

        rule ascii_small_letter_l() -> ()
            = ['\x6C'] { }

        rule ascii_small_letter_m() -> ()
            = ['\x6D'] { }

        rule ascii_small_letter_n() -> ()
            = ['\x6E'] { }

        rule ascii_small_letter_o() -> ()
            = ['\x6F'] { }

        rule ascii_small_letter_p() -> ()
            = ['\x70'] { }

        rule ascii_small_letter_q() -> ()
            = ['\x71'] { }

        rule ascii_small_letter_r() -> ()
            = ['\x72'] { }

        rule ascii_small_letter_s() -> ()
            = ['\x73'] { }

        rule ascii_small_letter_t() -> ()
            = ['\x74'] { }

        rule ascii_small_letter_u() -> ()
            = ['\x75'] { }

        rule ascii_small_letter_v() -> ()
            = ['\x76'] { }

        rule ascii_small_letter_w() -> ()
            = ['\x77'] { }

        rule ascii_small_letter_x() -> ()
            = ['\x78'] { }

        rule ascii_small_letter_y() -> ()
            = ['\x79'] { }

        rule ascii_small_letter_z() -> ()
            = ['\x7A'] { }

        rule ascii_left_curly_bracket() -> ()
            = ['\x7B'] { }

        rule ascii_vertical_line() -> ()
            = ['\x7C'] { }

        rule ascii_right_curly_bracket() -> ()
            = ['\x7D'] { }

        rule ascii_tilde() -> ()
            = ['\x7E'] { }

        rule ascii_delete() -> ()
            = ['\x7F'] { }

        rule ascii_crlf() -> ()
            = ['\x0D' | '\x0A'] { }

        rule ascii_digit() -> u8
            = ascii_digit_zero() { 0 }
            / ascii_digit_one() { 1 }
            / ascii_digit_two() { 2 }
            / ascii_digit_three() { 3 }
            / ascii_digit_four() { 4 }
            / ascii_digit_five() { 5 }
            / ascii_digit_six() { 6 }
            / ascii_digit_seven() { 7 }
            / ascii_digit_eight() { 8 }
            / ascii_digit_nine() { 9 }

        rule ascii_digit_hexadecimal() -> u8 // Line 151
            = ascii_digit_zero() { 0 }
            / ascii_digit_one() { 1 }
            / ascii_digit_two() { 2 }
            / ascii_digit_three() { 3 }
            / ascii_digit_four() { 4 }
            / ascii_digit_five() { 5 }
            / ascii_digit_six() { 6 }
            / ascii_digit_seven() { 7 }
            / ascii_digit_eight() { 8 }
            / ascii_digit_nine() { 9 }
            / ascii_capital_letter_a() { 10 }
            / ascii_small_letter_a() { 10 }
            / ascii_capital_letter_b() { 11 }
            / ascii_small_letter_b() { 11 }
            / ascii_capital_letter_c() { 12 }
            / ascii_small_letter_c() { 12 }
            / ascii_capital_letter_d() { 13 }
            / ascii_small_letter_d() { 13 }
            / ascii_capital_letter_e() { 14 }
            / ascii_small_letter_e() { 14 }
            / ascii_capital_letter_f() { 15 }
            / ascii_small_letter_f() { 15 }
    }
}

pub struct NumfmtParser<'source> {
    src: &'source str,
}

impl<'source> NumfmtParser<'source> {
    pub fn new(src: &'source str) -> Self {
        NumfmtParser { src }
    }

    pub fn parse(&self) -> PResult<NumFormat> {
        numfmt_parser::all(self.src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fmtstr(s: &str) -> PResult<NumFormat> {
        NumfmtParser::new(s).parse()
    }

    #[test]
    fn test_one() {
        let res = parse_fmtstr("@@");
        println!("{:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_builtin_formats() {
        let builtin_ids = [0];
        for &id in builtin_ids.iter() {
            let fmt: Option<&'static NumFormat> = builtin_format(id);
            assert!(fmt.is_some(), "Builtin format with ID {} not found", id);
        }
    }
}
