mod builtin;
pub mod formatter;
pub mod parser;
pub mod types;

pub use builtin::{builtin_format, builtin_formats};
pub use formatter::{FormatError, FormatResult, FormatValue, LocaleConfig, format};
pub use parser::{NumfmtParser, PResult};
pub use types::NumFormat;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fmtstr(s: &str) -> PResult<NumFormat> {
        NumfmtParser::new(s).parse()
    }

    #[test]
    fn test_one() {
        let res = parse_fmtstr("#,##0.00;(#,##0.00);\"Zero\"");
        println!(
            "{}",
            serde_json::to_string_pretty(res.as_ref().unwrap()).unwrap()
        );
        assert!(res.is_ok());
    }

    #[test]
    fn test_builtin_formats() {
        let builtin_ids = [
            0, 1, 2, 3, 4, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
        ];
        for &id in builtin_ids.iter() {
            let fmt: Option<&'static NumFormat> = builtin_format(id);
            assert!(fmt.is_some(), "Builtin format with ID {} not found", id);
            let fmt_pretty = serde_json::to_string_pretty(fmt.unwrap()).unwrap();
            println!("{}", fmt_pretty);
        }
    }
}
