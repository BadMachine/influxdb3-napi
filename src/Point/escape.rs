use std::collections::HashMap;

pub struct Escaper {
    char_map: HashMap<char, String>,
}

impl Escaper {
    pub fn new(characters: &str, replacements: Vec<&str>) -> Self {
        let char_map = characters
            .chars()
            .zip(replacements.into_iter().map(String::from))
            .collect();

        Self { char_map }
    }

    pub fn escape(&self, value: &str) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        for (i, ch) in value.char_indices() {
            if let Some(replacement) = self.char_map.get(&ch) {
                result.push_str(&value[last_end..i]);
                result.push_str(replacement);
                last_end = i + ch.len_utf8();
            }
        }

        if last_end == 0 {
            value.to_string()
        } else {
            result.push_str(&value[last_end..]);
            result
        }
    }

    pub fn escape_measurement() -> Self {
        Escaper::new(", \n\r\t", vec!["\\,", "\\ ", "\\n", "\\r", "\\t"])
    }

    pub fn escape_quoted() -> Self {
        Escaper::new("\"\\", vec!["\\\"", "\\\\"])
    }

    pub fn escape_tag() -> Self {
        Escaper::new(", =\n\r\t", vec!["\\,", "\\ ", "\\=", "\\n", "\\r", "\\t"])
    }
}