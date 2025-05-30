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

    #[test]
    fn test_nfp_formats() {
        let nfp = std::fs::read_to_string("src/nfp.txt").unwrap();
        let nfp = nfp.split('\n');
        let mut errors = Vec::new();
        
        for (i, line) in nfp.enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            match parse_fmtstr(line) {
                Ok(_) => {},
                Err(e) => {
                    println!("Failed to parse format string at line {}: {}", i, line);
                    println!("Error: {:?}", e);
                    errors.push(format!("{}: {} - {:?}", i, line, e));
                }
            }
        }
        
        if !errors.is_empty() {
            std::fs::write("failed_formats.txt", errors.join("\n")).unwrap();
            println!("Wrote {} errors to failed_formats.txt", errors.len());
        }
    }
}
