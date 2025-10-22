use std::collections::HashMap;
use std::fmt;
use tera::{Error as TeraError, Result as TeraResult, Value};

#[derive(Debug, Clone)]
enum ColorFilterError {
    Type {
        expected: &'static str,
    },
    HexLength {
        actual: usize,
    },
    HexDigit {
        component: &'static str,
        value: String,
    },
    AlphaRange {
        value: f64,
    },
}

impl fmt::Display for ColorFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type { expected } => {
                write!(f, "Invalid type, expected {}", expected)
            }
            Self::HexLength { actual } => {
                write!(
                    f,
                    "Invalid hex code length: expected 6 characters, got {}",
                    actual
                )
            }
            Self::HexDigit { component, value } => {
                write!(
                    f,
                    "Invalid hex value for {} component: '{}'",
                    component, value
                )
            }
            Self::AlphaRange { value } => {
                write!(f, "Alpha value {} must be between 0.0 and 1.0", value)
            }
        }
    }
}

impl From<ColorFilterError> for TeraError {
    fn from(err: ColorFilterError) -> Self {
        TeraError::msg(err.to_string())
    }
}

/// Adds a '#' prefix to a hex color string.
///
/// # Examples
///
/// In a Tera template:
/// ```text
/// {{ "FF5733" | hex_hash }}  -> "#FF5733"
/// ```
///
/// # Errors
///
/// Returns an error if the input is not a string.
pub fn hex_hash(value: &Value, _args: &HashMap<String, Value>) -> TeraResult<Value> {
    let color_str = value
        .as_str()
        .ok_or(ColorFilterError::Type { expected: "string" })?;

    Ok(Value::String(format!("#{}", color_str)))
}

/// Converts a hex color code to RGB or RGBA format.
///
/// # Arguments
///
/// * `value` - A hex color string (with or without '#' prefix)
/// * `args` - Optional arguments:
///   - `a`: Alpha channel (0.0-1.0), defaults to 1.0
///
/// # Examples
///
/// In a Tera template:
/// ```text
/// {{ "FF5733" | rgb }}           -> "rgb(255, 87, 51)"
/// {{ "#2E8B57" | rgb }}          -> "rgb(46, 139, 87)"
/// {{ "4169E1" | rgb(a=0.5) }}    -> "rgba(65, 105, 225, 0.50)"
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Input is not a string
/// - Hex code is not exactly 6 characters (after removing '#')
/// - Hex contains invalid characters
/// - Alpha value is outside [0.0, 1.0] range
pub fn rgb(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let alpha = args.get("a").and_then(|v| v.as_f64()).unwrap_or(1.0);

    if !(0.0..=1.0).contains(&alpha) {
        return Err(ColorFilterError::AlphaRange { value: alpha }.into());
    }

    let hex_str = value
        .as_str()
        .ok_or(ColorFilterError::Type { expected: "string" })?;

    let hex_code = hex_str.strip_prefix('#').unwrap_or(hex_str);

    if hex_code.len() != 6 {
        return Err(ColorFilterError::HexLength {
            actual: hex_code.len(),
        }
        .into());
    }

    let r = parse_hex_component(hex_code, 0..2, "Red")?;
    let g = parse_hex_component(hex_code, 2..4, "Green")?;
    let b = parse_hex_component(hex_code, 4..6, "Blue")?;

    let output = format_rgb_output(r, g, b, alpha);
    Ok(Value::String(output))
}

#[inline]
fn parse_hex_component(
    hex_code: &str,
    range: std::ops::Range<usize>,
    component_name: &'static str,
) -> Result<u8, TeraError> {
    let hex_slice = &hex_code[range.clone()];
    u8::from_str_radix(hex_slice, 16).map_err(|_| {
        ColorFilterError::HexDigit {
            component: component_name,
            value: hex_slice.to_string(),
        }
        .into()
    })
}

#[inline]
fn format_rgb_output(r: u8, g: u8, b: u8, alpha: f64) -> String {
    const EPSILON: f64 = 1e-10;

    if (alpha - 1.0).abs() < EPSILON {
        format!("rgb({}, {}, {})", r, g, b)
    } else {
        format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha)
    }
}
