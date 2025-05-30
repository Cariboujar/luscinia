//! Datetime formatter implementation

use crate::formatter::LocaleConfig;
use crate::formatter::error::{FormatError, FormatResult};
use crate::types::datetime::*;
use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike};

/// Format a datetime value according to DatetimeTuple format specification
pub fn format_datetime(value: f64, format: &DatetimeTuple, locale: &LocaleConfig) -> FormatResult {
    let datetime = excel_serial_to_datetime(value)?;

    let mut result = String::new();

    if let Some(dt_part1) = &format.0 {
        result.push_str(&format_nf_datetime(&datetime, dt_part1, locale)?);
    }
    if format.1.is_some() {
        result.push_str(&value.to_string());
    }
    if let Some(dt_part2) = &format.2 {
        result.push_str(&format_nf_datetime(&datetime, dt_part2, locale)?);
    }

    Ok(result)
}

/// Format a datetime according to NFDatetime format specification
pub fn format_nf_datetime(
    datetime: &DateTime<Local>,
    format: &NFDatetime,
    locale: &LocaleConfig,
) -> FormatResult {
    let has_ampm = format
        .components
        .iter()
        .any(|comp| matches!(comp, NFDatetimeComponent::AMPM(_)));
    let mut result = String::new();

    for component in &format.components {
        match component {
            NFDatetimeComponent::Token(token) => {
                if let NFDateTimeToken::Hour(fmt) = token {
                    if has_ampm {
                        let mut hour_12 = datetime.hour() % 12;
                        if hour_12 == 0 {
                            hour_12 = 12;
                        }
                        result.push_str(&format_hour(hour_12 as i32, *fmt)?);
                    } else {
                        result.push_str(&format_datetime_token(datetime, token, locale)?);
                    }
                } else {
                    result.push_str(&format_datetime_token(datetime, token, locale)?);
                }
            }
            NFDatetimeComponent::DateSeparator(c) => {
                result.push(*c);
            }
            NFDatetimeComponent::TimeSeparator(c) => {
                result.push(*c);
            }
            NFDatetimeComponent::AMPM(ampm) => {
                let is_pm = datetime.hour() >= 12;
                result.push_str(&format_ampm(ampm, is_pm));
            }
            NFDatetimeComponent::Literal(text) => {
                result.push_str(text);
            }
        }
    }

    Ok(result)
}

/// Format a datetime token
pub fn format_datetime_token(
    datetime: &DateTime<Local>,
    token: &NFDateTimeToken,
    locale: &LocaleConfig,
) -> FormatResult {
    match token {
        NFDateTimeToken::Year(fmt) => format_year(datetime.year(), fmt),
        NFDateTimeToken::Month(fmt) => format_month(datetime.month() as i32, *fmt, locale),
        NFDateTimeToken::Day(fmt) => format_day(
            datetime.day() as i32,
            datetime.weekday().num_days_from_sunday(),
            *fmt,
            locale,
        ),
        NFDateTimeToken::Hour(fmt) => {
            let hour = datetime.hour();
            format_hour(hour as i32, *fmt)
        }
        NFDateTimeToken::Minute(fmt) => format_minute(datetime.minute() as i32, *fmt),
        NFDateTimeToken::Second(fmt) => format_second(datetime.second() as i32, *fmt),
        NFDateTimeToken::SubSecond(fmt) => format_subsecond(datetime, fmt),
        NFDateTimeToken::EraG(_fmt) => {
            // jp era
            Ok("平成".to_string())
        }
        NFDateTimeToken::EraYear(fmt) => {
            // Era year - simplified implementation
            format_era_year(datetime.year(), *fmt)
        }
        NFDateTimeToken::CalendarB(_fmt) => {
            // Calendar type - simplified implementation
            Ok("1".to_string())
        }
        NFDateTimeToken::Abs(abs_token) => format_abs_time_token(datetime, abs_token),
    }
}

/// Format year component
fn format_year(year: i32, fmt: &YearFormat) -> FormatResult {
    match fmt {
        YearFormat::TwoDigit => Ok(format!("{:02}", year % 100)),
        YearFormat::FourDigit => Ok(format!("{:04}", year)),
    }
}

/// Format month component
fn format_month(month: i32, fmt: MonthFormat, _locale: &LocaleConfig) -> FormatResult {
    match fmt.0 {
        1 => Ok(format!("{}", month)),
        2 => Ok(format!("{:02}", month)),
        3 => {
            // Month abbreviation (e.g., "Jan")
            let month_names = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ];
            Ok(month_names[(month - 1) as usize].to_string())
        }
        4 => {
            // Full month name (e.g., "January")
            let month_names = [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ];
            Ok(month_names[(month - 1) as usize].to_string())
        }
        5 => {
            // First letter of month (e.g., "J")
            let month_names = ["J", "F", "M", "A", "M", "J", "J", "A", "S", "O", "N", "D"];
            Ok(month_names[(month - 1) as usize].to_string())
        }
        _ => Err(FormatError::FormatError("Invalid month format".to_string())),
    }
}

/// Format day component
fn format_day(day: i32, weekday: u32, fmt: DayFormat, _locale: &LocaleConfig) -> FormatResult {
    match fmt.0 {
        1 => Ok(format!("{}", day)),
        2 => Ok(format!("{:02}", day)),
        3 => {
            // Weekday abbreviation (e.g., "Mon")
            let weekday_names = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            Ok(weekday_names[weekday as usize].to_string())
        }
        4 => {
            // Full weekday name (e.g., "Monday")
            let weekday_names = [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ];
            Ok(weekday_names[weekday as usize].to_string())
        }
        _ => Err(FormatError::FormatError("Invalid day format".to_string())),
    }
}

/// Format hour component
fn format_hour(hour: i32, fmt: HourFormat) -> FormatResult {
    match fmt {
        HourFormat::OneChar => Ok(format!("{}", hour)),
        HourFormat::TwoChar => Ok(format!("{:02}", hour)),
    }
}

/// Format minute component
fn format_minute(minute: i32, fmt: MinuteFormat) -> FormatResult {
    match fmt {
        MinuteFormat::OneChar => Ok(format!("{}", minute)),
        MinuteFormat::TwoChar => Ok(format!("{:02}", minute)),
    }
}

/// Format second component
fn format_second(second: i32, fmt: SecondFormat) -> FormatResult {
    match fmt {
        SecondFormat::OneChar => Ok(format!("{}", second)),
        SecondFormat::TwoChar => Ok(format!("{:02}", second)),
    }
}

/// Format subsecond component
fn format_subsecond(datetime: &DateTime<Local>, fmt: &SubSecondFormat) -> FormatResult {
    let millis = datetime.nanosecond() / 1_000_000;

    match fmt.0 {
        1 => Ok(format!(".{}", millis / 100)),
        2 => Ok(format!(".{:02}", millis / 10)),
        3 => Ok(format!(".{:03}", millis)),
        _ => Ok(format!(".{:03}", millis)),
    }
}

/// Format era year component
fn format_era_year(year: i32, fmt: EraYearFormat) -> FormatResult {
    // Simplified implementation for era year
    match fmt {
        EraYearFormat::OneDigit => Ok(format!("{}", year % 100)),
        EraYearFormat::TwoDigit => Ok(format!("{}", year)),
    }
}

/// Format absolute time token
fn format_abs_time_token(datetime: &DateTime<Local>, token: &AbsTimeToken) -> FormatResult {
    match token {
        AbsTimeToken::AbsHour(fmt) => {
            // Total number of hours (for durations)
            let total_hours = datetime.hour() as i32;
            format_abs_value(total_hours, fmt.0)
        }
        AbsTimeToken::AbsMinute(fmt) => {
            // Total number of minutes (for durations)
            let total_minutes = datetime.hour() as i32 * 60 + datetime.minute() as i32;
            format_abs_value(total_minutes, fmt.0)
        }
        AbsTimeToken::AbsSecond(fmt) => {
            // Total number of seconds (for durations)
            let total_seconds = (datetime.hour() as i32 * 60 + datetime.minute() as i32) * 60
                + datetime.second() as i32;
            format_abs_value(total_seconds, fmt.0)
        }
    }
}

/// Format absolute time value with specified number of digits
fn format_abs_value(value: i32, num_digits: u8) -> FormatResult {
    let format_str = format!("{:0width$}", value, width = num_digits as usize);
    Ok(format_str)
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

/// Convert Excel serial date to DateTime
fn excel_serial_to_datetime(serial: f64) -> Result<DateTime<Local>, FormatError> {
    let days = serial.trunc() as i64;
    let time_fraction = serial.fract();
    let adjusted_days = if days > 60 { days - 1 } else { days };
    let base_date = match NaiveDateTime::parse_from_str("1899-12-31 00:00:00", "%Y-%m-%d %H:%M:%S")
    {
        Ok(dt) => dt,
        Err(_) => {
            return Err(FormatError::FormatError(
                "Failed to parse base date".to_string(),
            ));
        }
    };

    let date_part = base_date + chrono::Duration::days(adjusted_days);

    let seconds = (time_fraction * 86400.0).round() as i64; // 86400 seconds in a day
    let time_part = date_part + chrono::Duration::seconds(seconds);

    // Convert to local timezone
    let datetime = match Local.from_local_datetime(&time_part) {
        chrono::offset::LocalResult::Single(dt) => dt,
        _ => {
            return Err(FormatError::FormatError(
                "Failed to convert to local time".to_string(),
            ));
        }
    };

    Ok(datetime)
}
