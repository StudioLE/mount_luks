use crate::prelude::*;
use error_stack::ResultExt;
use rpassword::prompt_password;
use std::fs::read_to_string;
use std::io::Write;
use std::process::{Child, Stdio};

pub fn unlock_luks(options: &Options) -> Result<(), Report<UnlockError>> {
    let password = read_password(options)?;
    check_password(options, &password)?;
    unlock_luks_inner(options, &password)?;
    Ok(())
}

pub fn read_password(options: &Options) -> Result<String, Report<UnlockError>> {
    let Some(path) = &options.key_path else {
        return prompt_password("Enter LUKS password: ").change_context(UnlockError::Prompt);
    };
    read_to_string(path)
        .change_context(UnlockError::Read)
        .attach(format!("Path: {}", path.display()))
}

pub fn check_password(options: &Options, password: &str) -> Result<(), Report<UnlockError>> {
    let mut child = Command::new("cryptsetup")
        .arg("luksOpen")
        .arg("--test-passphrase")
        .arg("--key-file=-")
        .arg(options.partition_path.display().to_string())
        .stdin(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `cryptsetup luksOpen --test-passphrase`");
    write_to_stdin(&mut child, password);
    let status = child
        .wait()
        .expect("should be able to wait on `cryptsetup luksOpen --test-passphrase`");
    if status.success() {
        Ok(())
    } else {
        bail!(UnlockError::InvalidPassword);
    }
}

pub fn unlock_luks_inner(options: &Options, password: &str) -> Result<(), Report<UnlockError>> {
    let mut child = Command::new("cryptsetup")
        .arg("luksOpen")
        .arg("--key-file=-") // Read password from stdin
        .arg(options.partition_path.display().to_string())
        .arg(&options.mapper_name)
        .stdin(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `cryptsetup luksOpen`");
    write_to_stdin(&mut child, password);
    let status = child
        .wait()
        .expect("should be able to spawn `cryptsetup luksOpen`");
    if status.success() {
        Ok(())
    } else {
        bail!(UnlockError::LuksError)
    }
}

pub fn write_to_stdin(child: &mut Child, password: &str) {
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(password.as_bytes())
            .expect("Should be able to write to stdin");
    }
}

#[derive(Debug, Error)]
pub enum UnlockError {
    #[error("Unable to read password from prompt")]
    Prompt,
    #[error("Unable to read key file")]
    Read,
    #[error("Password is incorrect")]
    InvalidPassword,
    #[error("Failed to unlock LUKS partition")]
    LuksError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _read_password() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");
        assert!(options.key_path.is_some(), "Test requires key file");

        // Act
        let result = read_password(&options);

        // Assert
        if is_root().is_ok() {
            assert!(result.is_ok());
        } else if let Err(report) = &result {
            eprintln!("{report:?}");
            let _error = report
                .downcast_ref::<UnlockError>()
                .expect("should be UnlockError");
        }
    }
}
