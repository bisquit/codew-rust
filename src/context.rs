use std::{path::PathBuf};

pub struct Context {

}

impl Context {
  pub fn get_home_dir() -> Option<PathBuf> {
    dirs::home_dir()
  }

  pub fn get_codew_home_dir() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join(".codew"))
  }
}
