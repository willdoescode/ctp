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

    #[error("Parse error")]
    ParseError,
}

fn base_toml_checks(toml_value: &toml::Value) -> Result<(), TomlError> {
    let toml_table_unwraped = toml_value.as_table().unwrap();
    if !toml_table_unwraped.contains_key("templates") {
        return Err(TomlError::SectionNotFound("templates".into()));
    }

    if !toml_table_unwraped.contains_key("commands-before") {
        return Err(TomlError::SectionNotFound("commands-before".into()));
    }

    if !toml_table_unwraped.contains_key("commands-after") {
        return Err(TomlError::SectionNotFound("commands-after".into()));
    }

    Ok(())
}

fn get_table<'a>(
    toml_value: &toml::Value,
    name: &str,
) -> Result<toml::map::Map<String, toml::Value>, TomlError> {
    let templates = match toml_value[name].as_table() {
        Some(table) => table,
        None => return Err(TomlError::SectionNotFound(name.into())),
    };

    Ok(templates.to_owned())
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
        Ok(s) => Ok(s),
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
) -> Result<Vec<String>, TomlError> {
    base_toml_checks(toml_value)?;
    let commands = get_table(toml_value, &variant.to_string())?;
    let commands = extract_language_value(&commands, lang_name)?;

    Ok(commands)
}

pub fn get_lang_location(toml_value: &toml::Value, lang_name: &str) -> Result<String, TomlError> {
    base_toml_checks(toml_value)?;
    let templates = get_table(toml_value, "templates")?;
    let language = extract_language_value(&templates, lang_name)?;

    Ok(language)
}
