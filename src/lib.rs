mod builtin;
pub mod parser;
pub mod types;

pub use builtin::{builtin_format, builtin_formats};
pub use parser::{NumfmtParser, PResult};
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fmtstr(s: &str) -> PResult<NumFormat> {
        NumfmtParser::new(s).parse()
    }

    #[test]
    fn test_one() {
        let res = parse_fmtstr("mm-dd-yy");
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
