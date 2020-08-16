pub mod types;

/// Convert a byte to string.
pub fn as_string(byte: u8) -> String {
  let mut chars = vec![];
  chars.push(byte);
  String::from_utf8(chars).unwrap()
}

/// Comprobate if the byte is a single or double quote.
pub fn is_quote(byte: u8) -> bool {
  let value = as_string(byte);
  let value = value.as_str();

  value == "\"" || value == "'"
}

/// Comprobate if the byte is a letter.
pub fn is_letter(byte: u8) -> bool {
  let character = as_string(byte);
  let character = character.as_str();

  byte.is_ascii_alphabetic() || character == "_" || character == "$"
}

/// Comprobate if the byte is a digit.
pub fn is_digit(byte: u8) -> bool {
  byte.is_ascii_digit()
}

/// Repeat a character and get the final string.
/// 
/// ## Example
/// ```
/// use crate::utils::repeat_character;
/// 
/// let message = repeat_character(4, "-");
/// // Returns: "----"
/// ```
pub fn repeat_character(size: usize, character: &str) -> String {
  let mut message = String::new();

  while message.len() < size {
    message.push_str(character);
  }

  message
}
