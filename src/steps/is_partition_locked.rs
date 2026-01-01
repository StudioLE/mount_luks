use crate::prelude::*;

pub fn is_partition_locked(options: &Options) -> Result<(), Report<PartitionUnlocked>> {
    let mapper_path = options.get_mapper_path();
    if mapper_path.exists() {
        Err(Report::new(PartitionUnlocked).attach(format!(
            "Mapper device already exists: {}",
            mapper_path.display()
        )))
    } else {
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("Partition is already unlocked")]
pub struct PartitionUnlocked;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_partition_locked() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");

        // Act
        let result = is_partition_locked(&options);

        // Assert
        if let Err(report) = &result {
            eprintln!("{report:?}");
            let _error = report
                .downcast_ref::<PartitionUnlocked>()
                .expect("should be PartitionUnlocked");
        }
    }
}
