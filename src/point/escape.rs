use std::fmt;

// https://docs.influxdata.com/influxdb/cloud/reference/syntax/line-protocol/#special-characters
pub const COMMA_EQ_SPACE: [char; 3] = [',', '=', ' '];
pub const COMMA_SPACE: [char; 2] = [',', ' '];
pub const DOUBLE_QUOTE: [char; 1] = ['"'];


pub fn escape<const N: usize>(src: &str, special_characters: [char; N]) -> Escaped<'_, N> {
    Escaped {
        src,
        special_characters,
    }
}

pub struct Escaped<'a, const N: usize> {
    src: &'a str,
    special_characters: [char; N],
}

impl<const N: usize> fmt::Display for Escaped<'_, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.src.chars() {
            if self.special_characters.contains(&ch) || ch == '\\' {
                write!(f, "\\")?;
            }
            write!(f, "{ch}")?;
        }
        Ok(())
    }
}