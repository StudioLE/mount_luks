use crate::prelude::*;
use dirs::config_dir;
use dotenvy::from_path_iter;
use error_stack::ResultExt;
use std::collections::HashMap;

pub struct Options {
    pub partition_path: PathBuf,
    pub mapper_name: String,
    pub mount_path: PathBuf,
    pub key_path: Option<PathBuf>,
}

impl Options {
    pub fn read_options() -> Result<Options, Report<OptionsError>> {
        let path = config_dir()
            .expect("should be able to get config directory")
            .join("mount_luks")
            .join(".env");
        if !path.exists() {
            let report =
                Report::new(OptionsError::NoFile).attach(format!("Path: {}", path.display()));
            return Err(report);
        }
        let file = from_path_iter(path).change_context(OptionsError::Read)?;
        let vars: HashMap<String, String> = file.filter_map(Result::ok).collect();
        Ok(Options {
            partition_path: PathBuf::from(get_value(&vars, "PARTITION_PATH")?),
            mapper_name: get_value(&vars, "MAPPER_NAME")?,
            mount_path: PathBuf::from(get_value(&vars, "MOUNT_PATH")?),
            key_path: vars.get("KEY_PATH").map(PathBuf::from),
        })
    }

    pub fn get_mapper_path(&self) -> PathBuf {
        PathBuf::from("/dev/mapper").join(&self.mapper_name)
    }
}

fn get_value(
    hash_map: &HashMap<String, String>,
    key: &str,
) -> Result<String, Report<OptionsError>> {
    hash_map
        .get(key)
        .ok_or_else(|| Report::new(OptionsError::Required).attach(format!("Key: {}", key)))
        .map(|s| s.to_string())
}

#[derive(Debug, Error)]
pub enum OptionsError {
    #[error("Options file does not exist")]
    NoFile,
    #[error("Unable to read options file")]
    Read,
    #[error("Options file is not complete")]
    Required,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _read_options() {
        // Arrange

        // Act
        let _options = Options::read_options().expect("Should be able to read options");

        // Assert
    }
}
