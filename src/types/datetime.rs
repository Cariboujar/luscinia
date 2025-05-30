use crate::types::NFGeneral;
use serde::Serialize;

/// [NFDateTime] [NFGeneral] [NFDateTime]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DatetimeTuple(
    pub Option<NFDatetime>,
    pub Option<NFGeneral>,
    pub Option<NFDatetime>,
);

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NFDatetime {
    pub components: Vec<NFDatetimeComponent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum NFDatetimeComponent {
    Token(NFDateTimeToken),
    DateSeparator(char),
    TimeSeparator(char),
    AMPM(AmPm),
    Literal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum AmPm {
    Full,
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AbsTimeToken {
    AbsHour(AbsHourFormat),
    AbsMinute(AbsMinuteFormat),
    AbsSecond(AbsSecondFormat),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
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
    SubSecond(SubSecondFormat),
    Abs(AbsTimeToken),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EraFormatG {
    OneDigit,
    TwoDigit,
    ThreeDigit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CalendarTypeB {
    Gregorian,
    Hijri,
}

/// 1-5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MonthFormat(pub u8);

impl MonthFormat {
    pub(crate) fn from_minute_format(minute_format: MinuteFormat) -> Self {
        match minute_format {
            MinuteFormat::OneChar => MonthFormat(1),
            MinuteFormat::TwoChar => MonthFormat(2),
        }
    }
}

/// 1-4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DayFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum HourFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbsHourFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum MinuteFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbsMinuteFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u8)]
pub enum SecondFormat {
    OneChar,
    TwoChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AbsSecondFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SubSecondFormat(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum YearFormat {
    TwoDigit,
    FourDigit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EraYearFormat {
    OneDigit,
    TwoDigit,
}
