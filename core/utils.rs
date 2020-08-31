use std::env;

pub fn get_sflyn_path() -> String {
  match env::var("SFLYN_PATH") {
    Ok(sflyn_path) => sflyn_path,
    Err(_) => String::new()
  }
}

/// Repeat a character and get the final string.
/// 
/// ## Example
/// ```
/// use sflyn::utils::repeat_character;
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
