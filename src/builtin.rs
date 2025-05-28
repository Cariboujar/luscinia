use crate::parser::NumfmtParser;
use crate::types::NumFormat;
use std::collections::BTreeMap;
use std::sync::OnceLock;

pub static BUILTIN_FORMATS: OnceLock<BTreeMap<u8, NumFormat>> = OnceLock::new();

pub fn builtin_formats() -> &'static BTreeMap<u8, NumFormat> {
    BUILTIN_FORMATS.get_or_init(|| {
        include_str!("builtin.tsv")
            .lines()
            .map(|line| {
                let mut parts = line.split('\t');
                let id = parts.next().unwrap().parse::<u8>().unwrap();
                let str_format = parts.next().unwrap();
                println!("{}", str_format);
                let format = NumfmtParser::new(str_format).parse().unwrap();
                (id, format)
            })
            .collect()
    })
}

pub fn builtin_format(id: u8) -> Option<&'static NumFormat> {
    builtin_formats().get(&id)
}
