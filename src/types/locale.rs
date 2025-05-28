use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
