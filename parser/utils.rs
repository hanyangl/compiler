/// Repeat a character and get the final string.
/// 
/// ## Example
/// ```
/// use sflyn_parser::utils::repeat_character;
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
