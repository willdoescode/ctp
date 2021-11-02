use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("\"template\" section could not be found in your config. Add it with [template]")]
    TemplatesNotFound,

    #[error("The language \"{0}\" could not be found in your config.")]
    LanguageNotFound(String),

    #[error("The value of \"{0}\" is an invalid type, expected String")]
    InvalidType(String),

    #[error("Parse error")]
    ParseError,
}

pub fn get_lang_location(toml_value: &toml::Value, lang_name: &str) -> Result<String, TomlError> {
    if !toml_value.as_table().unwrap().contains_key("templates") {
        return Err(TomlError::TemplatesNotFound);
    }

    let templates = match toml_value["templates"].as_table() {
        Some(table) => table,
        None => return Err(TomlError::ParseError),
    };

    if !templates.contains_key(lang_name) {
        return Err(TomlError::LanguageNotFound(lang_name.into()));
    }

    if !templates[lang_name].is_str() {
        return Err(TomlError::InvalidType(lang_name.into()));
    }

    let language = match templates[lang_name].as_str() {
        Some(s) => s,
        None => return Err(TomlError::ParseError),
    }
    .to_string();

    Ok(language)
}
