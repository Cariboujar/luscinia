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

pub type Any = MaybeColored<TextOr<MaybeConditional<NumberOrFracOrDt>>>;
pub type AnyNoText = MaybeColored<MaybeConditional<NumberOrFracOrDt>>;
pub type AnyNoCond = MaybeColored<TextOr<NumberOrFracOrDt>>;
pub type AnyNoTextNoCond = MaybeColored<NumberOrFracOrDt>;

/// [NFDateTime] [NFGeneral] [NFDateTime]
#[derive(Debug, PartialEq, Eq)]
pub struct DatetimeTuple(
    pub Option<NFDatetime>,
    pub Option<NFGeneral>,
    pub Option<NFDatetime>,
);

#[derive(Debug, PartialEq, Eq)]
pub struct NFDatetime {
    pub components: Vec<NFDatetimeComponent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NFDatetimeComponent {
    Token(NFDateTimeToken),
    SubSecond(SubSecondFormat),
    DateSeparator,
    TimeSeparator,
    AMPM(AmPm),
    Literal(char),
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

#[derive(Debug, PartialEq, Eq)]
pub struct NFGeneral {}

#[derive(Debug, PartialEq, Eq)]
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

/// true => @
/// false => INTL-AMPM
#[derive(Debug, PartialEq, Eq)]
pub struct NFText {
    pub format: Vec<bool>,
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
#[derive(Debug)]
pub struct NFPartCondition {
    pub op: NFCondOperator,
    pub value: f64,
}

impl PartialEq for NFPartCondition {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && (self.value - other.value).abs() < f64::EPSILON
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
    pub name: String,
    pub suffix: Option<Vec<u8>>,
}
