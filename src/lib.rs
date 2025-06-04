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
    #[ignore]
    fn test_from_pipe() {
        use std::io::{self, Read};
        
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
        let input = input.trim();
        
        if input.is_empty() {
            println!("No input provided. Please pipe a format string to this test.");
            return;
        }
        
        println!("Parsing format string: {}", input);
        let res = parse_fmtstr(input);
        
        match &res {
            Ok(format) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(format).unwrap()
                );
            },
            Err(e) => {
                println!("Error parsing format string: {:?}", e);
            }
        }
        
        assert!(res.is_ok(), "Failed to parse format string: {:?}", res.err().unwrap());
    }
    
    #[test]
    fn test_nfp_formats() {
        let nfp = std::fs::read_to_string("src/fixture/nfp.txt").unwrap();
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
            std::fs::write("nfp_failed_formats.txt", errors.join("\n")).unwrap();
            println!("Wrote {} errors to nfp_failed_formats.txt", errors.len());
        }
    }

    #[test]
    fn test_andersnm_valid_formats() {
        let andersnm_valid = std::fs::read_to_string("src/fixture/andersnm_valid.txt").unwrap();
        let andersnm_valid = andersnm_valid.split('\n');
        let mut errors = Vec::new();
        
        for (i, line) in andersnm_valid.enumerate() {
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
            std::fs::write("andersnm_failed_formats.txt", errors.join("\n")).unwrap();
            println!("Wrote {} errors to andersnm_failed_formats.txt", errors.len());
        }
    }

    #[test]
    fn test_special_prefixes() {
        let test_cases = vec![
            "[ENG][$-409]d\\-mmm;@",
            "[Red]#.##",
            "[MAGENTA]0.00",
            "#\\ ??/100;[Red]\\(#\\ ??/16\\)",
        ];
        
        for case in test_cases {
            println!("Testing format: {}", case);
            let res = parse_fmtstr(case);
            
            match &res {
                Ok(format) => {
                    println!(
                        "Successfully parsed:\n{}",
                        serde_json::to_string_pretty(format).unwrap()
                    );
                },
                Err(e) => {
                    println!("Error parsing: {:?}", e);
                }
            }
            println!("---");
        }
    }

    #[test]
    fn test_failed_formats_sample() {
        let test_cases = vec![
            "[DBNum1][$-804]AM/PMh\"时\"mm\"分\";@",
            "[HIJ][$-2060401]d/mm/yyyy\\ h:mm\\ AM/PM;@", 
            "[JPN][$-411]gggyy\"年\"m\"月\"d\"日\"\\ dddd;@",
            "[TWN][DBNum1][$-404]y\"年\"m\"月\"d\"日\";@",
            "[WHITE]0.0",
            "[MAGENTA]0.00",
        ];
        
        for case in test_cases {
            println!("Testing format: {}", case);
            let res = parse_fmtstr(case);
            
            match &res {
                Ok(_) => {
                    println!("✅ Successfully parsed");
                },
                Err(e) => {
                    println!("❌ Error parsing: {:?}", e);
                }
            }
            println!("---");
        }
    }
}
