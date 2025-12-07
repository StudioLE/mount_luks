use crate::prelude::*;

pub fn check_mount_exists(options: &Options) -> Result<(), Report<NoMount>> {
    if options.mount_path.exists() {
        Ok(())
    } else {
        Err(Report::new(NoMount).attach(format!("Path: {}", options.mount_path.display())))
    }
}

#[derive(Debug, Error)]
#[error("Mount point does not exist")]
pub struct NoMount;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _check_mount_exists() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");
        
        // Act
        let result = check_mount_exists(&options);

        // Assert
        if let Err(report) = result {
            eprintln!("{report:?}");
            let _error = report.downcast_ref::<NoMount>().expect("should be NoMount");
        }
    }
}

