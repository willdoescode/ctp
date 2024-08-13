use anyhow::Result;
use thiserror::Error;
use toml::value::Table;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("\"{0}\" section could not be found in your config. Add it with [{0}]")]
    SectionNotFound(String),

    #[error("The language \"{0}\" could not be found in your config.")]
    LanguageNotFound(String),

    #[error("The value of \"{0}\" is an invalid type, expected String")]
    InvalidType(String),
}

fn base_toml_checks(toml_value: &toml::Value) -> Result<(), TomlError> {
    toml_value
        .as_table()
        .ok_or_else(|| TomlError::SectionNotFound("root".into()))
        .and_then(|table| {
            if !table.contains_key("templates") {
                return Err(TomlError::SectionNotFound("templates".into()));
            }

            Ok(())
        })
}

fn get_table<'i, 'a>(toml_value: &'a toml::Value, name: &str) -> Result<&'i Table, TomlError>
where
    'a: 'i,
{
    toml_value
        .get(name)
        .and_then(|val| val.as_table())
        .ok_or_else(|| TomlError::SectionNotFound(name.into()))
}

fn extract_language_value<'a, T>(table: &'a Table, lang_name: &str) -> Result<T, TomlError>
where
    T: toml::macros::Deserialize<'a>,
{
    let value = table
        .get(lang_name)
        .ok_or_else(|| TomlError::LanguageNotFound(lang_name.into()))?;

    value
        .clone()
        .try_into()
        .map_err(|_| TomlError::InvalidType(lang_name.into()))
}

#[derive(Debug, Clone, Copy)]
pub enum CommandVariants {
    Before,
    After,
}

impl ToString for CommandVariants {
    fn to_string(&self) -> String {
        match self {
            Self::Before => "commands-before",
            Self::After => "commands-after",
        }
        .into()
    }
}

pub fn get_commands(
    toml_value: &toml::Value,
    lang_name: &str,
    variant: CommandVariants,
) -> Result<Option<Vec<String>>, TomlError> {
    base_toml_checks(toml_value)?;

    if let Some(commands) = get_table(toml_value, &variant.to_string()).ok() {
        if let Ok(commands) = extract_language_value(commands, lang_name) {
            return Ok(Some(commands));
        }
    }

    Ok(None)
}

pub fn get_lang_location(toml_value: &toml::Value, lang_name: &str) -> Result<String, TomlError> {
    base_toml_checks(toml_value)?;

    let templates = get_table(toml_value, "templates")?;
    extract_language_value(&templates, lang_name)
}
