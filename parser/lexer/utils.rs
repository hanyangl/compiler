/// Convert a character to a str.
pub fn character_to_str(character: u8) -> &'static str {
  let mut characters = Vec::new();
  characters.push(character);

  match String::from_utf8(characters) {
    Ok(string) => Box::leak(string.into_boxed_str()),
    Err(_) => "",
  }
}

/// Check if a character is a valid letter for an identifier.
pub fn is_letter_identifier(character: u8) -> bool {
  let character_as_str = character_to_str(character);

  character.is_ascii_alphabetic() || character_as_str == "_" || character_as_str == "$"
}

/// Check if a character is a valid number.
pub fn is_number(character: u8) -> bool {
  character.is_ascii_digit()
}
