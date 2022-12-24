use std::fmt;

#[derive(Debug, Clone)]
pub struct TokenizerError;

impl fmt::Display for TokenizerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "token error")
  }
}