use crate::mount_partition;
use crate::prelude::*;

pub fn app() {
    init_elapsed_logger();
    if let Err(e) = internal() {
        print_error("Unable to continue");
        eprintln!("\n{e}");
    }
}

fn internal() -> Result<(), AnyReport> {
    let options = Options::read_options()?;

    print_header(&options);
    let counter = Mutex::new(0);

    print_step_start(&counter, "Checking if root");
    is_root()?;
    print_step_completed("Access granted");

    print_step_start(&counter, "Checking if partition exists");
    check_partition_exist(&options)?;
    print_step_completed("Partition exists");

    print_step_start(&counter, "Checking if partition is encrypted with LUKS");
    is_luks_partition(&options)?;
    print_step_completed("Partition is encrypted with LUKS");

    print_step_start(&counter, "Checking if partition is already unlocked");
    is_partition_locked(&options)?;
    print_step_completed("Partition is locked");

    print_step_start(&counter, "Unlocking LUKS partition");
    unlock_luks(&options)?;
    print_step_completed("Unlocked LUKS partition");

    print_step_start(&counter, "Checking mount point exists");
    check_mount_exists(&options)?;
    print_step_completed("Mount point exists");

    print_step_start(&counter, "Checking if already mounted");
    check_if_mounted(&options)?;
    print_step_completed("Partition is not mounted");

    print_step_start(&counter, "Mounting partition");
    mount_partition(&options)?;
    print_step_completed("Partition mounted successfully");

    Ok(())
}
