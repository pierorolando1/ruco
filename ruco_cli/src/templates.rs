
pub const TEMPLATE_MAIN_CONTENT: &str = r#"require std

fu main |arg1, arg2| do
  std::print("Hello from {{NAME}}")
ck
"#;

pub const TEMPLATE_CONFIG_CONTENT: &str = r#"[project]
name = "{{NAME}}"
version = "0.1.0"

[runtime]
# Commented options are the defualt options 
#version = "defualt"
#entrypoint = "main.ruco"

[scripts]
start = "ruco run"
"#;
