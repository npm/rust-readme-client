extern crate npm_readme_client;

use npm_readme_client::Error;

pub fn readme() -> Result<String, Error> {
  Ok(String::from(r#"<!-- this HTML was generated using marky-markdown version 11.2.0. see an issue? file at https://github.com/npm/marky-markdown/issues. please include the version in your issue. thanks for using marky! to learn more, visit https://github.com/npm/marky-markdown.-->"#))
}

