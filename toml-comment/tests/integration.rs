use serde::Serialize;
use toml_comment::TomlComment;

/// Application settings
#[derive(Serialize, TomlComment)]
struct AppConfig {
    /// The application name
    name: String,
    /// Whether debug mode is enabled
    debug: bool,
    /// Maximum retry count
    max_retries: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "myapp".to_string(),
            debug: false,
            max_retries: 3,
        }
    }
}

#[test]
fn basic_struct() {
    let toml = AppConfig::default_toml();
    let expected = "\
# Application settings
# The application name
name = \"myapp\"
# Whether debug mode is enabled
debug = false
# Maximum retry count
max_retries = 3
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct ServerConfig {
    /// Port to listen on
    port: u16,
    /// Bind address
    host: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
        }
    }
}

/// Root config
#[derive(Serialize, TomlComment)]
struct RootConfig {
    server: ServerConfig,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
}

#[test]
fn nested_struct() {
    let toml = RootConfig::default_toml();
    let expected = "\
# Root config

[server]
# Port to listen on
port = 8080
# Bind address
host = \"127.0.0.1\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithOption {
    /// Always present
    name: String,
    /// Sometimes present
    extra: Option<String>,
}

impl Default for WithOption {
    fn default() -> Self {
        Self {
            name: "hello".to_string(),
            extra: None,
        }
    }
}

#[test]
fn option_none_omitted() {
    let toml = WithOption::default_toml();
    let expected = "\
# Always present
name = \"hello\"
";
    assert_eq!(toml, expected);
}

#[test]
fn option_some_included() {
    let cfg = WithOption {
        name: "hello".to_string(),
        extra: Some("world".to_string()),
    };
    let toml = cfg.to_commented_toml();
    let expected = "\
# Always present
name = \"hello\"
# Sometimes present
extra = \"world\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct WithVec {
    /// Allowed origins
    origins: Vec<String>,
}

impl Default for WithVec {
    fn default() -> Self {
        Self {
            origins: vec!["localhost".to_string(), "example.com".to_string()],
        }
    }
}

#[test]
fn vec_field() {
    let toml = WithVec::default_toml();
    let expected = "\
# Allowed origins
origins = [\"localhost\", \"example.com\"]
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct NoComments {
    name: String,
    count: u32,
}

impl Default for NoComments {
    fn default() -> Self {
        Self {
            name: "test".to_string(),
            count: 42,
        }
    }
}

#[test]
fn no_comment_fields() {
    let toml = NoComments::default_toml();
    let expected = "\
name = \"test\"
count = 42
";
    assert_eq!(toml, expected);
}

#[test]
fn non_default_values() {
    let cfg = AppConfig {
        name: "custom".to_string(),
        debug: true,
        max_retries: 10,
    };
    let toml = cfg.to_commented_toml();
    assert!(toml.contains("name = \"custom\""));
    assert!(toml.contains("debug = true"));
    assert!(toml.contains("max_retries = 10"));
}

#[derive(Serialize, TomlComment)]
struct LoggingConfig {
    /// Log level
    level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Serialize, TomlComment)]
struct DatabaseConfig {
    /// Connection URL
    url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://data.db".to_string(),
        }
    }
}

/// Multi-section config
#[derive(Serialize, TomlComment)]
struct MultiSection {
    logging: LoggingConfig,
    database: DatabaseConfig,
}

impl Default for MultiSection {
    fn default() -> Self {
        Self {
            logging: LoggingConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}

#[test]
fn multiple_sections_separated_by_blank_line() {
    let toml = MultiSection::default_toml();
    let expected = "\
# Multi-section config

[logging]
# Log level
level = \"info\"

[database]
# Connection URL
url = \"sqlite://data.db\"
";
    assert_eq!(toml, expected);
}

#[derive(Serialize, TomlComment)]
struct Thresholds {
    /// Temperature threshold
    temperature: f64,
    /// Ratio value
    ratio: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            ratio: 0.75,
        }
    }
}

#[test]
fn float_formatting() {
    let toml = Thresholds::default_toml();
    assert!(toml.contains("temperature = 1.0"));
    assert!(toml.contains("ratio = 0.75"));
}
