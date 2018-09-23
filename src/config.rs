//! Configuration save & load utility

// Parse config using crate toml
extern crate toml;

// Error handling
use std::error::Error;

// Reading file from filesystem
use std::fs::File;
use std::io::prelude::*;

// Bot configuration
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Config {
    pub host: Option<String>,
    pub base_url: Option<String>,
    pub template: Option<String>,
    pub root: Option<String>,
    pub file_size: Option<bool>
}

impl Config {
    /// Generate new config
    pub fn new(host: &String, base_url: &String ,template: &String, root: &String, file_size: &bool) -> Result<Config, &'static str> {
        Ok(
            Config {
                host: Some(host.clone()),
                base_url: Some(base_url.clone()),
                template: Some(template.clone()),
                root: Some(root.clone()),
                file_size: Some(file_size.clone())
            }
        )
    }

    /// Write config to file at given location
    pub fn write(&self, location: &String) -> Result<(), Box<Error>> {
        let content = toml::to_string(&self)?;
        let mut buffer = File::create(location)?;
        buffer.write(content.as_bytes())?;
        Ok(())
    }
}

/// Reads config (TOML) from file
pub fn parse_config(config_filename: String) -> Result<Config, Box<Error>> {
    debug!("Reading config from: {}", config_filename);
    // Read from file
    let mut file = File::open(config_filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    debug!("Got: {}", content);
    // Parsing
    let config: Config = toml::from_str(&content)?;
    info!("Autonomous systems are go.");
    Ok(config)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    // Test config parsing functionality
    #[test]
    fn config_read() {
        if fs::metadata("test_config_read.toml").is_ok() {
            fs::remove_file("test_config_read.toml").unwrap();
        }
        {
            let mut buffer = fs::File::create("test_config_read.toml").unwrap();
            buffer.write(b"host = \"localhost:8090\"\nbase_url = \"http://localhost:8090\"\ntemplate = \"./template\"\nroot = \"./demo\"\nfile_size = true\n").unwrap();
            assert_eq!(
                parse_config("test_config_read.toml".to_string()).unwrap(),
                Config {
                    host: Some("localhost:8090".to_string()),
                    base_url: Some("http://localhost:8090".to_string()),
                    template: Some("./template".to_string()),
                    root: Some("./demo".to_string()),
                    file_size: Some(true)
                }
            );
        }
        if fs::metadata("test_config_read.toml").is_ok() {
            fs::remove_file("test_config_read.toml").unwrap();
        }
    }

    // Test config generating functionality
    #[test]
    fn config_generate() {
        assert_eq!(
            Config::new(&"localhost:8090".to_string(), &"http://localhost:8090".to_string(),&"./template".to_string(), &"./demo".to_string(), &true).unwrap(),
            Config {
                host: Some("localhost:8090".to_string()),
                base_url: Some("http://localhost:8090".to_string()),
                template: Some("./template".to_string()),
                root: Some("./demo".to_string()),
                file_size: Some(true)
            }
        );
    }

    // Test config writing functionality
    #[test]
    fn config_write() {
        if fs::metadata("test_config_write.toml").is_ok() {
            fs::remove_file("test_config_write.toml").unwrap();
        }
        {
            let config = Config::new(&"localhost:8090".to_string(), &"http://localhost:8090".to_string(), &"./template".to_string(), &"./demo".to_string(), &true).unwrap();
            config.write(&"test_config_write.toml".to_string()).unwrap();
            let mut file = fs::File::open("test_config_write.toml").unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            assert_eq!(
                content,
                "host = \"localhost:8090\"\nbase_url = \"http://localhost:8090\"\ntemplate = \"./template\"\nroot = \"./demo\"\nfile_size = true\n".to_string()
            )
        }
        if fs::metadata("test_config_write.toml").is_ok() {
            fs::remove_file("test_config_write.toml").unwrap();
        }
    }
}
