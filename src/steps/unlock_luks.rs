use crate::prelude::*;
use error_stack::ResultExt;
use rpassword::prompt_password;
use std::io::Write;
use std::process::{Child, Stdio};

pub fn unlock_luks(options: &Options) -> Result<(), Report<UnlockError>> {
    let password = read_password()?;
    check_password(options, &password)?;
    unlock_luks_inner(options, &password)?;
    Ok(())
}

pub fn read_password() -> Result<String, Report<UnlockError>> {
    prompt_password("Enter LUKS password: ").change_context(UnlockError::PasswordError)
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
    #[error("Failed to read password")]
    PasswordError,
    #[error("Password is incorrect")]
    InvalidPassword,
    #[error("Failed to unlock LUKS partition")]
    LuksError,
}
