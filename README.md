# Luscinia

A spreadsheet number formatting syntax parser and formatter for Rust, following the [MS-OE376 2.1.739 Part 4 Section 3.8.30, numFmt (Number Format)](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/0e59abdb-7f4e-48fc-9b89-67832fa11789).

> [!NOTE]
> This crate is still in development and is not yet ready for production use.

## Features

- Parse and interpret complex number format strings
- Support for various number format components:
  - Numbers with custom decimal and thousand separators
  - Fractions
  - Text literals
  - Date and time formatting
  - Conditional formatting
- Locale-aware formatting (TODO)
- Color specifications (TODO)
- Built-in format templates

## Usage

### Basic Parsing

```rust
use luscinia::{NumfmtParser, NumFormat};

fn main() {
    // Parse a format string
    let format_string = "#,##0.00;(#,##0.00);\"Zero\"";
    let format = NumfmtParser::new(format_string).parse().unwrap();
    
    // Inspect the parsed format structure
    println!("{}", serde_json::to_string_pretty(&format).unwrap());
}
```

### Formatting Values

```rust
use luscinia::{format, FormatValue, LocaleConfig};

fn main() {
    let format_string = "#,##0.00;(#,##0.00);\"Zero\"";
    
    // Format a positive number
    let result = format(1234.56, format_string, &LocaleConfig::default()).unwrap();
    assert_eq!(result, "1,234.56");
    
    // Format a negative number (uses the second section)
    let result = format(-1234.56, format_string, &LocaleConfig::default()).unwrap();
    assert_eq!(result, "(1,234.56)");
    
    // Format zero (uses the third section)
    let result = format(0.0, format_string, &LocaleConfig::default()).unwrap();
    assert_eq!(result, "Zero");
}
```

### Using Built-in Formats

```rust
use luscinia::{builtin_format, format_with_parsed, FormatValue, LocaleConfig};

fn main() {
    // Use a built-in format by ID
    let format = builtin_format(0);
    println!("{}", serde_json::to_string_pretty(&format).unwrap());

    let result = format_with_parsed(FormatValue::Number(1234.56), format, &LocaleConfig::default()).unwrap();
    println!("{}", result);
}
```

## Format String Syntax

Luscinia supports a rich format string syntax inspired by spreadsheet applications:

- **Sections**: Separated by semicolons, for positive, negative, zero, and text values
- **Number formatting**: `#` (optional digit), `0` (required digit), `.` (decimal point)
- **Grouping**: `,` for thousand separators
- **Text**: Enclosed in double quotes `"text"`
- **Colors**: `[Red]`, `[Blue]`, etc.
- **Conditions**: `[>100]`, `[<=0]`, etc.
- **Locale**: Language and region specific formatting (TODO)

## Advanced Features

### Testing

The library includes tests for various formatting scenarios:

```rust
// Run all tests
cargo test

// Run a specific test
cargo test test_one
```

## Development

Enable trace debugging for the PEG parser:

```bash
cargo test --features trace
```

With pegviz:

```bash
echo '[$-ja-JP-x-gannen,80]' | cargo test test_from_pipe --features trace -- --ignored | pegviz --output pegviz.html
```
