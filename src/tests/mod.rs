use crate::commands::exec;
use crate::shape::{get_lang_location, TomlError};

#[test]
fn test_get_lang_location() {
    let missing_lang = r#"
    [templates]

  "#;

    let missing_lang_toml_value: toml::Value = toml::from_str(missing_lang).unwrap();
    let missing_lang_location = get_lang_location(&missing_lang_toml_value, "rust");

    if let Err(e @ TomlError::LanguageNotFound(..)) = missing_lang_location {
        assert_eq!(
            e.to_string(),
            "The language \"rust\" could not be found in your config.".to_string()
        )
    } else {
        panic!("Location given");
    }
}

#[test]
fn test_missing_templates() {
    let missing_lang = r#"
    [template]

  "#;

    let missing_templates: toml::Value = toml::from_str(missing_lang).unwrap();
    let missing_templates_output = get_lang_location(&missing_templates, "rust");

    assert!(matches!(
        missing_templates_output,
        Err(TomlError::TemplatesNotFound)
    ));
}

#[test]
fn test_invalid_type() {
    let missing_lang = r#"
    [templates]
    rust = 5
  "#;

    let invalid_type: toml::Value = toml::from_str(missing_lang).unwrap();
    let invalid_type_output = get_lang_location(&invalid_type, "rust");

    if let Err(e @ TomlError::InvalidType(..)) = invalid_type_output {
        assert_eq!(
            e.to_string(),
            "The value of \"rust\" is an invalid type, expected String".to_string()
        )
    }
}

#[test]
fn test_command() {
    let exec_output = exec("", "");
    match exec_output {
        Ok(_) => panic!("Expected error."),
        Err(e) => assert_eq!(e.to_string(), "Cannot execute empty command."),
    }
}
