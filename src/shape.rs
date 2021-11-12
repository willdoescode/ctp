use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TomlError {
    #[error("\"{0}\" section could not be found in your config. Add it with [{0}]")]
    SectionNotFound(String),

    #[error("The language \"{0}\" could not be found in your config.")]
    LanguageNotFound(String),

    #[error("The value of \"{0}\" is an invalid type, expected String")]
    InvalidType(String),

    #[error("Could not convert toml types")]
    ConversionError,
}

fn base_toml_checks(toml_value: &toml::Value) -> Result<(), TomlError> {
    let toml_table_unwrapped = match toml_value.as_table() {
        Some(t) => t,
        None => return Err(TomlError::ConversionError),
    };

    if !toml_table_unwrapped.contains_key("templates") {
        return Err(TomlError::SectionNotFound("templates".into()));
    }

    Ok(())
}

fn get_table<'a>(
    toml_value: &toml::Value,
    name: &str,
) -> Result<toml::map::Map<String, toml::Value>, TomlError> {
    match toml_value[name].as_table() {
        Some(table) => Ok(table.clone()),
        None => return Err(TomlError::SectionNotFound(name.into())),
    }
}

fn extract_language_value<'a, T>(
    table: &toml::map::Map<String, toml::Value>,
    lang_name: &str,
) -> Result<T, TomlError>
where
    T: toml::macros::Deserialize<'a>,
{
    if !table.contains_key(lang_name) {
        return Err(TomlError::LanguageNotFound(lang_name.into()));
    }

    match table[lang_name].to_owned().try_into::<T>() {
        Ok(val) => Ok(val),
        Err(_) => Err(TomlError::InvalidType(lang_name.into())),
    }
}

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
    let toml_value_table = match toml_value.as_table() {
        Some(t) => t,
        None => return Err(TomlError::ConversionError),
    };

    if !toml_value_table.contains_key(&variant.to_string()) {
        return Ok(None);
    }

    let commands = get_table(toml_value, &variant.to_string())?;
    match extract_language_value(&commands, lang_name) {
        Ok(commands) => Ok(Some(commands)),
        Err(_) => Ok(None),
    }
}

pub fn get_lang_location(toml_value: &toml::Value, lang_name: &str) -> Result<String, TomlError> {
    base_toml_checks(toml_value)?;
    let templates = get_table(toml_value, "templates")?;
    let language = extract_language_value(&templates, lang_name)?;

    Ok(language)
}
