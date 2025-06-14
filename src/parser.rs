use crate::types::*;
use peg::str::LineCol;

pub type PResult<T> = Result<T, peg::error::ParseError<LineCol>>;

pub struct NumfmtParser<'source> {
    src: &'source str,
}

impl<'source> NumfmtParser<'source> {
    pub fn new(src: &'source str) -> Self {
        NumfmtParser { src }
    }

    pub fn parse(&self) -> PResult<NumFormat> {
        numfmt_parser::toplevel(self.src)
    }
}

// The comment "Line xx" refers to the line number
// in the original ABNF specification.
//
// [MS-OE376] 2.1.739 Part 4 Section 3.8.30, numFmt (Number Format)
//
// https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/0e59abdb-7f4e-48fc-9b89-67832fa11789
peg::parser! {
    grammar numfmt_parser() for str {

        rule traced<T>(e: rule<T>) -> T =
            &(input:$([_]*) {
                #[cfg(feature = "trace")]
                println!("[PEG_INPUT_START]\n{}\n[PEG_TRACE_START]", input);
            })
            e:e()? {?
                #[cfg(feature = "trace")]
                println!("[PEG_TRACE_STOP]");
                e.ok_or("")
            }

        pub rule toplevel() -> NumFormat = traced(<all()>)

        pub rule all() -> NumFormat // Line 1
            = f1:nf_any() ascii_semicolon() f2:nf_any() ascii_semicolon() f3:nf_any_no_cond() ascii_semicolon() f4:all_f4()? {
                NumFormat::FourParts(
                    f1,
                    f2,
                    f3,
                    f4,
                )
            }
            / f1:nf_any() ascii_semicolon() f2:nf_any() ascii_semicolon() f3:nf_any_no_cond() {
                NumFormat::ThreeParts(
                    f1,
                    f2,
                    f3,
                )
            }
            / f1:nf_any() ascii_semicolon() f2:nf_any() {
                NumFormat::TwoParts(
                    f1,
                    f2,
                )
            }
            / f:nf_any_no_cond() { NumFormat::AnyNoCond(f) }
            / special_prefix:nf_part_special_prefix()* locale:nf_part_locale_id()? color:nf_part_color()? cond:nf_part_cond() g:nf_general() {
                NumFormat::ConditionalGeneral(SectionWrapper {
                    locale,
                    color,
                    special_prefix,
                    inner: (cond, g),
                })
            }
            / special_prefix:nf_part_special_prefix()* locale:nf_part_locale_id()? color:nf_part_color()? cond:nf_part_cond() g:nf_general() {
                NumFormat::ConditionalGeneral(SectionWrapper {
                    locale,
                    color,
                    special_prefix,
                    inner: (cond, g),
                })
            }

            rule all_f4() -> FormatComponent // Custom
                = t:nf_text() { FormatComponent::Text(t) }
                / f:nf_general() { FormatComponent::General() }

            rule format_component() -> FormatComponent // Custom
                = f:nf_fraction() { FormatComponent::Fraction(f) }
                / n:nf_number() { FormatComponent::Number(n) }
                / dt:datetime_tuple() { FormatComponent::Datetime(dt) }
                / t:nf_text() { FormatComponent::Text(t) }
                / g:nf_general() { FormatComponent::General() }

            rule datetime_tuple() -> DatetimeTuple // Custom
                = dt1:nf_datetime()? g:nf_general()? dt2:nf_datetime()? {?
                    if dt1.is_none() && dt2.is_none() {
                        return Err("At least one datetime must be present");
                    }
                    Ok(DatetimeTuple(dt1, g, dt2))
                }

        rule nf_any() -> SectionWrapper<AnyInner> // Line 2
            = special_prefix:nf_part_special_prefix()* locale:nf_part_locale_id()? color:nf_part_color()? condition:nf_part_cond()? data:format_component() {
                SectionWrapper {
                    locale,
                    color,
                    special_prefix,
                    inner: AnyInner::ConditionalData(condition, data),
                }
            }

        rule nf_any_no_cond() -> AnyNoCond // Line 4
            = special_prefix:nf_part_special_prefix()* locale:nf_part_locale_id()? color:nf_part_color()? data:format_component() {
                SectionWrapper {
                    locale,
                    color,
                    special_prefix,
                    inner: data,
                }
            }
            / f:nf_general() {
                SectionWrapper {
                    locale: None,
                    color: None,
                    special_prefix: vec![],
                    inner: FormatComponent::General(),
                }
            }

        rule nf_general() -> NFGeneral // Line 6
            = intl_numfmt_general() { NFGeneral {} }

        rule nf_number() -> NFNumber // Line 7
            = part1:nf_part_num() exp:(scientific_notation())? {
                let has_percent = part1.iter().any(|token| matches!(token, DigitPosOrOther::Other(Percent {})));
                NFNumber {
                    num_part: part1,
                    exp_part: exp,
                    has_percent,
                }
            }

            rule scientific_notation() -> (Sign, Vec<DigitPosOrOther<Percent>>)
                = quiet!{e:ascii_capital_letter_e() sgn:(ascii_plus_sign() { Sign::Plus } / ascii_hyphen_minus() { Sign::Minus }) part2:nf_part_num() { (sgn, part2) }}
                / expected!("scientific notation (E+n or E-n)")

        rule nf_datetime_token() -> NFDateTimeToken // Line 8
            = y:nf_part_year() { NFDateTimeToken::Year(y) }
            / g:nf_part_era_g() { NFDateTimeToken::EraG(g) }
            / e:nf_part_era_year() { NFDateTimeToken::EraYear(e) }
            / d:nf_part_day() { NFDateTimeToken::Day(d) }
            / h:nf_part_hour() { NFDateTimeToken::Hour(h) }
            / s:nf_part_second() { NFDateTimeToken::Second(s) }
            / ss:nf_part_sub_second() { NFDateTimeToken::SubSecond(ss) }
            / m:nf_part_month() { NFDateTimeToken::Month(m) }
            // / m:nf_part_minute() { NFDateTimeToken::Minute(m) }
            // minute can only be parsed in patterns below
            / cb:nf_part_calendar_b() { NFDateTimeToken::CalendarB(cb) }
            / a:nf_abs_time_token() { NFDateTimeToken::Abs(a) }

        rule nf_abs_time_token() -> AbsTimeToken // Line 9
            = h:nf_part_abs_hour() { AbsTimeToken::AbsHour(h) }
            / m:nf_part_abs_minute() { AbsTimeToken::AbsMinute(m) }
            / s:nf_part_abs_second() { AbsTimeToken::AbsSecond(s) }

        rule nf_datetime() -> NFDatetime // Line 10
            = ampms:intl_ampm()* components:(dt_token_or_component())+ {?
                let all_components = components.into_iter().flatten()
                    .chain(ampms.into_iter().map(NFDatetimeComponent::AMPM))
                    .collect::<Vec<_>>();
                if !all_components.iter().any(|c| matches!(c, NFDatetimeComponent::Token(_))) {
                    return Err("At least one token must be present");
                }
                Ok(NFDatetime { components: all_components })
            }

            rule dt_token_or_component() -> Vec<NFDatetimeComponent>
                = h_m:nf_pattern_hour_minute() { h_m }
                / m_s:nf_pattern_minute_second() { m_s }
                / m_d:nf_pattern_month_day() { m_d }
                / token:nf_datetime_token() { vec![NFDatetimeComponent::Token(token)] }
                / component:nf_datetime_component() { vec![component] }

            rule nf_pattern_hour_minute() -> Vec<NFDatetimeComponent>
                = h:nf_part_hour() components:nf_datetime_component()* m:nf_part_minute_format() {
                    let mut result = vec![NFDatetimeComponent::Token(NFDateTimeToken::Hour(h))];
                    result.extend(components);
                    result.push(NFDatetimeComponent::Token(NFDateTimeToken::Minute(m)));
                    result
                }

            rule nf_pattern_minute_second() -> Vec<NFDatetimeComponent>
                = m:nf_part_minute_format() components:nf_datetime_component()* s:nf_part_second() {
                    let mut result = vec![NFDatetimeComponent::Token(NFDateTimeToken::Minute(m))];
                    result.extend(components);
                    result.push(NFDatetimeComponent::Token(NFDateTimeToken::Second(s)));
                    result
                }

            rule nf_pattern_month_day() -> Vec<NFDatetimeComponent>
                = m:nf_part_minute_format() components:nf_datetime_component()* d:nf_part_day() {
                    let mut result = vec![NFDatetimeComponent::Token(NFDateTimeToken::Month(MonthFormat::from_minute_format(m)))];
                    result.extend(components);
                    result.push(NFDatetimeComponent::Token(NFDateTimeToken::Day(d)));
                    result
                }

            rule nf_datetime_component() -> NFDatetimeComponent // Custom
                = ampm:intl_ampm() { NFDatetimeComponent::AMPM(ampm) }
                / lit_str:literal_string() { NFDatetimeComponent::Literal(lit_str) }
                / date_sep:intl_char_date_sep() { NFDatetimeComponent::Literal(date_sep.to_string()) }
                / time_sep:intl_char_time_sep() { NFDatetimeComponent::Literal(time_sep.to_string()) }
                / ascii_space() { NFDatetimeComponent::Literal(" ".to_string()) }
                / ascii_comma() { NFDatetimeComponent::Literal(",".to_string()) }
                / bc:unmatched_literal_char() { NFDatetimeComponent::Literal(bc.to_string()) }

        rule nf_text() -> NFText // Line 11
            = elements:(nf_text_element())+ {
                NFText { elements }
            }

            rule nf_text_element() -> TextFormatElement
                = ascii_commercial_at() { TextFormatElement::AtPlaceholder }
                / ampm_val:intl_ampm() { TextFormatElement::AmPm(ampm_val) }
                / lcs:literal_char_space() { TextFormatElement::LiteralCharSpace(lcs) }
                / ls:literal_string() { TextFormatElement::LiteralString(ls) }
                / fc:literal_char_repeat() { TextFormatElement::FillChar(fc) }
                / ec:literal_char() { TextFormatElement::EscapedChar(ec) }
                / bc:unmatched_literal_char() { TextFormatElement::LiteralString(bc.to_string()) }

        rule nf_fraction() -> NFFraction // Line 12
            = prefix:nf_fraction_preffix_or_suffix_element()* int_part:nf_num_only() sep:nf_frac_separator()+ num:nf_part_fraction() ascii_space()* ascii_solidus() ascii_space()* denom:nf_part_fraction() suffix:nf_fraction_preffix_or_suffix_element()* ampm:intl_ampm()* {
                NFFraction {
                    prefix,
                    integer_part: Some(int_part),
                    separator: Some(sep),
                    numerator: num,
                    denominator: denom,
                    suffix,
                    ampm_part: ampm,
                }
            }
            / prefix:nf_fraction_preffix_or_suffix_element()* num:nf_part_fraction() ascii_space()* ascii_solidus() ascii_space()* denom:nf_part_fraction() suffix:nf_fraction_preffix_or_suffix_element()* ampm:intl_ampm()* {
                NFFraction {
                    prefix,
                    integer_part: None,
                    separator: None,
                    numerator: num,
                    denominator: denom,
                    suffix,
                    ampm_part: ampm,
                }
            }

            rule nf_num_only() -> Vec<DigitPosOrOther<Percent>>
                = tks:nf_part_num_token1()+ {
                    tks.into_iter().map(|t| DigitPosOrOther::Digit(DigitPos::Digit(t))).collect()
                }

            rule nf_frac_separator() -> DigitPosOrOther<Percent>
                = currency:nf_part_locale_id() { DigitPosOrOther::Currency(currency) }
                / lit_str:literal_string() { DigitPosOrOther::LiteralString(lit_str) }
                / ec:literal_char() { DigitPosOrOther::EscapedChar(ec) }
                / lcs:literal_char_space() { DigitPosOrOther::LiteralCharSpace(lcs) }
                / ascii_space()+ { DigitPosOrOther::LiteralCharSpace(' ') }

            rule nf_fraction_preffix_or_suffix_element() -> DigitPosOrOther<Percent>
                = ascii_percent_sign() { DigitPosOrOther::Other(Percent {}) }
                / currency:nf_part_locale_id() { DigitPosOrOther::Currency(currency) }
                / lcs_char:literal_char_space() { DigitPosOrOther::LiteralCharSpace(lcs_char) }
                / lit_str:literal_string() { DigitPosOrOther::LiteralString(lit_str) }
                / fill_char:literal_char_repeat() { DigitPosOrOther::FillChar(fill_char) }
                / esc_char:literal_char() { DigitPosOrOther::EscapedChar(esc_char) }
                / bare_char:unmatched_literal_char() { DigitPosOrOther::LiteralString(bare_char.to_string()) }

        rule nf_part_num() -> Vec<DigitPosOrOther<Percent>> // Line 13
            = tks:nf_format_element()+ {
                ?
                if tks.first().is_some_and(|t| matches!(t, DigitPosOrOther::Other(_)))
                    && tks.last().is_some_and(|t| matches!(t, DigitPosOrOther::Other(_)))
                {
                    Err("Invalid number format: percent sign at both ends")
                } else if !tks.iter().any(|t| matches!(t, DigitPosOrOther::Digit(_))) {
                    Err("Invalid number format: must contain at least one digit")
                } else {
                    Ok(tks)
                }
            }

            rule nf_format_element() -> DigitPosOrOther<Percent>
                = t:nf_part_num_token2() { DigitPosOrOther::Digit(t) }
                / ascii_percent_sign() { DigitPosOrOther::Other(Percent {}) }
                / currency:nf_part_locale_id() { DigitPosOrOther::Currency(currency) }
                / lcs_char:literal_char_space() { DigitPosOrOther::LiteralCharSpace(lcs_char) }
                / lit_str:literal_string() { DigitPosOrOther::LiteralString(lit_str) }
                / fill_char:literal_char_repeat() { DigitPosOrOther::FillChar(fill_char) }
                / esc_char:literal_char() { DigitPosOrOther::EscapedChar(esc_char) }
                / bare_char:unmatched_literal_char() { DigitPosOrOther::LiteralString(bare_char.to_string()) }

        rule nf_part_exponential() -> Sign // Line 14
            = ascii_capital_letter_e() sgn:nf_part_sign() { sgn }

        rule nf_part_year() -> YearFormat // Line 15
            = "yyyy" { YearFormat::FourDigit }
            / "YYYY" { YearFormat::FourDigit }
            / "yy" { YearFormat::TwoDigit }
            / "YY" { YearFormat::TwoDigit }

        rule nf_part_era_g() -> EraFormatG // Custom
            = "g" { EraFormatG::OneDigit }
            / "gg" { EraFormatG::TwoDigit }
            / "ggg" { EraFormatG::ThreeDigit }

        rule nf_part_era_year() -> EraYearFormat // Custom
            = "e" { EraYearFormat::OneDigit }
            / "ee" { EraYearFormat::TwoDigit }

        rule nf_part_calendar_b() -> CalendarTypeB // Custom
            = "b1" { CalendarTypeB::Gregorian }
            / "b2" { CalendarTypeB::Hijri }
            / expected!("calendar type (b1 or b2)")

        rule nf_part_month() -> MonthFormat // Line 16
            = m:(ascii_small_letter_m()*<1,5>) {
                MonthFormat(m.len() as u8)
            }
            / m:(ascii_capital_letter_m()*<1,5>) {
                MonthFormat(m.len() as u8)
            }

        rule nf_part_day() -> DayFormat // Line 17
            = d:(ascii_small_letter_d()*<1,4>) {
                DayFormat(d.len() as u8)
            }
            / d:(ascii_capital_letter_d()*<1,4>) {
                DayFormat(d.len() as u8)
            }

        rule nf_part_hour() -> HourFormat // Line 18
            = "hh" { HourFormat::TwoChar }
            / "HH" { HourFormat::TwoChar }
            / "h" { HourFormat::OneChar }
            / "H" { HourFormat::OneChar }

        rule nf_part_abs_hour() -> AbsHourFormat // Line 19
            = ascii_left_square_bracket() h:ascii_small_letter_h()+ ascii_right_square_bracket() {
                AbsHourFormat(h.len() as u8)
            }

        rule nf_part_minute() -> MinuteFormat // Line 20
            = "mm" { MinuteFormat::TwoChar }
            / "MM" { MinuteFormat::TwoChar }
            / "m" { MinuteFormat::OneChar }
            / "M" { MinuteFormat::OneChar }

        rule nf_part_minute_format() -> MinuteFormat // Helper rule to parse m/mm without interpreting it
            = "mm" { MinuteFormat::TwoChar }
            / "MM" { MinuteFormat::TwoChar }
            / "m" { MinuteFormat::OneChar }
            / "M" { MinuteFormat::OneChar }

        rule nf_part_abs_minute() -> AbsMinuteFormat // Line 21
            = ascii_left_square_bracket() m:ascii_small_letter_m()+ ascii_right_square_bracket() {
                AbsMinuteFormat(m.len() as u8)
            }

        rule nf_part_second() -> SecondFormat // Line 22
            = "ss" { SecondFormat::TwoChar }
            / "SS" { SecondFormat::TwoChar }
            / "s" { SecondFormat::OneChar }
            / "S" { SecondFormat::OneChar }

        rule nf_part_abs_second() -> AbsSecondFormat // Line 23
            = ascii_left_square_bracket() s:ascii_small_letter_s()+ ascii_right_square_bracket() {
                AbsSecondFormat(s.len() as u8)
            }

        rule nf_part_sub_second() -> SubSecondFormat // Line 24
            = intl_char_decimal_sep() z:(ascii_digit_zero()*<1,3>) {?
                Ok(SubSecondFormat(z.len() as u8))
            }
            / expected!("sub-second (.0, .00, or .000)")

        rule nf_part_cond() -> NFPartCondition // Line 25
            = ascii_left_square_bracket() op:nf_part_comp_oper() value:nf_part_cond_num() ascii_right_square_bracket() {
                NFPartCondition { op, value }
            }

        rule nf_part_comp_oper() -> NFCondOperator // Line 26
            = ascii_equals_sign() { NFCondOperator::Equal }
            / ascii_less_than_sign() ascii_greater_than_sign() { NFCondOperator::NotEqual }
            / ascii_greater_than_sign() ascii_equals_sign() {
                NFCondOperator::GreaterThanOrEqual
            }
            / ascii_greater_than_sign() { NFCondOperator::GreaterThan }
            / ascii_less_than_sign() ascii_equals_sign() {
                NFCondOperator::LessThanOrEqual
            }
            / ascii_less_than_sign() { NFCondOperator::LessThan }

        rule nf_part_locale_id() -> PartLocaleID // Line 27
            = quiet!{
                ascii_left_square_bracket()
                ascii_dollar_sign()
                name_chars:currency_symbol_char()*
                hex_digits_opt:(ascii_hyphen_minus() v:nf_part_locale_id_hex_value() {v})?
                ascii_right_square_bracket()
                {
                    PartLocaleID::from_parsed_peg(name_chars, hex_digits_opt)
                }
            }
            / expected!("locale/currency format (e.g., [$-409] or [$USD-409])")

            rule currency_symbol_char() -> char
                = !(['-'] / [']']) c:utf16_any() { c }

            rule nf_part_locale_id_hex_value() -> Vec<u8>
                = val:ascii_digit_hexadecimal()*<3,8> { val }


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
            // / ascii_left_square_bracket() nf_part_str_color() id:nf_part_1to56() ascii_right_square_bracket() {
            //     NFPartColor::Color(id)
            // }

            // Use uint + string literal
            / "[" nf_part_str_color() id:uint() "]" {
                ? if id > 56 || id == 0 {
                    Err("Color ID must be between 1 and 56")
                } else {
                    Ok(NFPartColor::Color(id as u8))
                }
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

        rule nf_part_fraction() -> Vec<FracToken> // Line 35
            = tokens:(
                (t:nf_part_num_token1() { FracToken::Placeholder(t) })
                / (ascii_percent_sign() { FracToken::Percent })
                / (d:ascii_digit() { FracToken::Digit(d) })
                )+ {?
                // Check if it's all percent signs with no placeholders or digits
                if tokens.iter().all(|t| matches!(t, FracToken::Percent)) {
                    Err("Fraction part must contain at least one #, ?, 0, or digit")
                } else {
                    let number_like = tokens.iter().all(|t| {
                        matches!(t, FracToken::Digit(_)) || matches!(t, FracToken::Placeholder(NumPlaceholder::Zero))
                    });
                    let has_digit = tokens.iter().any(|t| matches!(t, FracToken::Digit(_)));

                    if number_like && has_digit {
                        let mut value = 0u32;
                        for token in &tokens {
                            match token {
                                FracToken::Digit(d) => {
                                    value = value * 10 + (*d as u32);
                                },
                                FracToken::Placeholder(NumPlaceholder::Zero) => {
                                    value *= 10; // Zero placeholder treated as digit 0
                                },
                                _ => {} // Shouldn't occur due to the looks_like_number check
                            }
                        }
                        Ok(vec![FracToken::Number(value)])
                    } else {
                        Ok(tokens)
                    }
                }
            }
            / expected!("fraction part (#, ?, 0, digits, %)")

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
            // = ascii_capital_letter_c() ascii_small_letter_o() ascii_small_letter_l() ascii_small_letter_o() ascii_small_letter_r() { }
            = "Color" { }
            // zh_CN L10n
            / "颜色" { }

        rule literal_char() -> char // Line 40
            = ascii_reverse_solidus() c:utf16_any() { c }

        rule literal_char_repeat() -> char // Line 41
            = ascii_asterisk() c:utf16_any() { c }

        rule literal_string() -> String // Line 42
            = ascii_quotation_mark() s:utf16_any_without_quote()* ascii_quotation_mark() { s.iter().collect() }
            / s:literal_char()+ { s.iter().collect() }

        rule utf16_any_without_quote() -> char // Line 43
            = ['\u{0000}'..='\u{0021}' | '\u{0023}'..='\u{FFFF}']

        rule literal_char_space() -> char // Line 44
            = ascii_low_line() c:utf16_any() { c }

        rule intl_char_decimal_sep() -> () // Line 45
            = ascii_full_stop()

        rule intl_char_numgrp_sep() -> () // Line 46
            = ascii_comma()

        rule intl_char_date_sep() -> char // Line 47
            = ascii_solidus() { '/' }
            / ascii_hyphen_minus() { '-' }

        rule intl_char_time_sep() -> char // Line 48
            = ascii_colon() { ':' }

        rule intl_color() -> DefinedColor // Line 49
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
            = "AM/PM" { AmPm::Full }
            / "A/P" { AmPm::Simple }
            / expected!("AM/PM or A/P")

        rule utf16_any() -> char // Line 52
            = c:(['\u{0000}'..='\u{FFFF}']) { c }

        rule unmatched_literal_char() -> char
            = !nf_general() !nf_datetime_token() !intl_ampm() !nf_part_num_token1()
            !nf_abs_time_token() !nf_part_num_token2() !ascii_solidus() !ascii_commercial_at()
            !ascii_semicolon() !nf_part_exponential() !nf_part_fraction() c:utf16_any() { c }

        rule ascii_space() -> ()
            = [' '] { }

        rule ascii_exclamation_mark() -> ()
            = ['!'] { }

        rule ascii_quotation_mark() -> ()
            = ['"'] { }

        rule ascii_number_sign() -> ()
            = ['#'] { }

        rule ascii_dollar_sign() -> ()
            = ['$'] { }

        rule ascii_percent_sign() -> ()
            = ['%'] { }

        rule ascii_ampersand() -> ()
            = ['&'] { }

        rule ascii_apostrophe() -> ()
            = ['\''] { }

        rule ascii_left_parenthesis() -> ()
            = ['('] { }

        rule ascii_right_parenthesis() -> ()
            = [')'] { }

        rule ascii_asterisk() -> ()
            = ['*'] { }

        rule ascii_plus_sign() -> ()
            = ['+'] { }

        rule ascii_comma() -> ()
            = [','] { }

        rule ascii_hyphen_minus() -> ()
            = ['-'] { }

        rule ascii_full_stop() -> ()
            = ['.'] { }

        rule ascii_solidus() -> ()
            = ['/'] { }

        rule ascii_digit_zero() -> ()
            = ['0'] { }

        rule ascii_digit_one() -> ()
            = ['1'] { }

        rule ascii_digit_two() -> ()
            = ['2'] { }

        rule ascii_digit_three() -> ()
            = ['3'] { }

        rule ascii_digit_four() -> ()
            = ['4'] { }

        rule ascii_digit_five() -> ()
            = ['5'] { }

        rule ascii_digit_six() -> ()
            = ['6'] { }

        rule ascii_digit_seven() -> ()
            = ['7'] { }

        rule ascii_digit_eight() -> ()
            = ['8'] { }

        rule ascii_digit_nine() -> ()
            = ['9'] { }

        rule ascii_colon() -> ()
            = [':'] { }

        rule ascii_semicolon() -> ()
            = [';'] { }

        rule ascii_less_than_sign() -> ()
            = ['<'] { }

        rule ascii_equals_sign() -> ()
            = ['='] { }

        rule ascii_greater_than_sign() -> ()
            = ['>'] { }

        rule ascii_question_mark() -> ()
            = ['?'] { }

        rule ascii_commercial_at() -> ()
            = ['@'] { }

        rule ascii_capital_letter_a() -> ()
            = ['A'] { }

        rule ascii_capital_letter_b() -> ()
            = ['B'] { }

        rule ascii_capital_letter_c() -> ()
            = ['C'] { }

        rule ascii_capital_letter_d() -> ()
            = ['D'] { }

        rule ascii_capital_letter_e() -> ()
            = ['E'] { }

        rule ascii_capital_letter_f() -> ()
            = ['F'] { }

        rule ascii_capital_letter_g() -> ()
            = ['G'] { }

        rule ascii_capital_letter_h() -> ()
            = ['H'] { }

        rule ascii_capital_letter_i() -> ()
            = ['I'] { }

        rule ascii_capital_letter_j() -> ()
            = ['J'] { }

        rule ascii_capital_letter_k() -> ()
            = ['K'] { }

        rule ascii_capital_letter_l() -> ()
            = ['L'] { }

        rule ascii_capital_letter_m() -> ()
            = ['M'] { }

        rule ascii_capital_letter_n() -> ()
            = ['N'] { }

        rule ascii_capital_letter_o() -> ()
            = ['O'] { }

        rule ascii_capital_letter_p() -> ()
            = ['P'] { }

        rule ascii_capital_letter_q() -> ()
            = ['Q'] { }

        rule ascii_capital_letter_r() -> ()
            = ['R'] { }

        rule ascii_capital_letter_s() -> ()
            = ['S'] { }

        rule ascii_capital_letter_t() -> ()
            = ['T'] { }

        rule ascii_capital_letter_u() -> ()
            = ['U'] { }

        rule ascii_capital_letter_v() -> ()
            = ['V'] { }

        rule ascii_capital_letter_w() -> ()
            = ['W'] { }

        rule ascii_capital_letter_x() -> ()
            = ['X'] { }

        rule ascii_capital_letter_y() -> ()
            = ['Y'] { }

        rule ascii_capital_letter_z() -> ()
            = ['Z'] { }

        rule ascii_left_square_bracket() -> ()
            = ['['] { }

        rule ascii_reverse_solidus() -> ()
            = ['\\'] { }

        rule ascii_right_square_bracket() -> ()
            = [']'] { }

        rule ascii_circumflex_accent() -> ()
            = ['^'] { }

        rule ascii_low_line() -> ()
            = ['_'] { }

        rule ascii_grave_accent() -> ()
            = ['`'] { }

        rule ascii_small_letter_a() -> ()
            = ['a'] { }

        rule ascii_small_letter_b() -> ()
            = ['b'] { }

        rule ascii_small_letter_c() -> ()
            = ['c'] { }

        rule ascii_small_letter_d() -> ()
            = ['d'] { }

        rule ascii_small_letter_e() -> ()
            = ['e'] { }

        rule ascii_small_letter_f() -> ()
            = ['f'] { }

        rule ascii_small_letter_g() -> ()
            = ['g'] { }

        rule ascii_small_letter_h() -> ()
            = ['h'] { }

        rule ascii_small_letter_i() -> ()
            = ['i'] { }

        rule ascii_small_letter_j() -> ()
            = ['j'] { }

        rule ascii_small_letter_k() -> ()
            = ['k'] { }

        rule ascii_small_letter_l() -> ()
            = ['l'] { }

        rule ascii_small_letter_m() -> ()
            = ['m'] { }

        rule ascii_small_letter_n() -> ()
            = ['n'] { }

        rule ascii_small_letter_o() -> ()
            = ['o'] { }

        rule ascii_small_letter_p() -> ()
            = ['p'] { }

        rule ascii_small_letter_q() -> ()
            = ['q'] { }

        rule ascii_small_letter_r() -> ()
            = ['r'] { }

        rule ascii_small_letter_s() -> ()
            = ['s'] { }

        rule ascii_small_letter_t() -> ()
            = ['t'] { }

        rule ascii_small_letter_u() -> ()
            = ['u'] { }

        rule ascii_small_letter_v() -> ()
            = ['v'] { }

        rule ascii_small_letter_w() -> ()
            = ['w'] { }

        rule ascii_small_letter_x() -> ()
            = ['x'] { }

        rule ascii_small_letter_y() -> ()
            = ['y'] { }

        rule ascii_small_letter_z() -> ()
            = ['z'] { }

        rule ascii_left_curly_bracket() -> ()
            = ['{'] { }

        rule ascii_vertical_line() -> ()
            = ['|'] { }

        rule ascii_right_curly_bracket() -> ()
            = ['}'] { }

        rule ascii_tilde() -> ()
            = ['~'] { }

        rule ascii_delete() -> ()
            = ['\x7F'] { }

        rule ascii_crlf() -> ()
            = ['\r' | '\n'] { }

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

        // Custom Part

        rule uint() -> u128
            = digits:ascii_digit()+ {
                digits.iter().fold(
                    0u128,
                    |acc, &d| acc * 10 + d as u128
                )
            }

        rule nf_part_special_prefix() -> String
            = "t" { "t".to_string() }
            / "[" prefix:known_special_prefix() "]" { prefix }

        rule known_special_prefix() -> String
            = "ENG" { "ENG".to_string() }
            / "DBNum1" { "DBNum1".to_string() }
            / "DBNum2" { "DBNum2".to_string() }
            / "DBNum3" { "DBNum3".to_string() }
            / "HIJ" { "HIJ".to_string() }
            / "JPN" { "JPN".to_string() }
            / "TWN" { "TWN".to_string() }
            / "MAGENTA" { "MAGENTA".to_string() }
            / "WHITE" { "WHITE".to_string() }
            / "CYAN" { "CYAN".to_string() }
            / "BLACK" { "BLACK".to_string() }
            / "BLUE" { "BLUE".to_string() }
            / "GREEN" { "GREEN".to_string() }
            / "YELLOW" { "YELLOW".to_string() }
            / !intl_color() !nf_part_str_color() !['>'] !['<'] !['='] !['$'] chars:utf16_any()+ {
                chars.into_iter().collect()
            }

        rule unknown_prefix_char() -> char
            = ![']'] c:utf16_any() { c }
    }
}
