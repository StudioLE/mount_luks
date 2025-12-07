use crate::prelude::*;
use owo_colors::OwoColorize;

const STEPS_COUNT: usize = 8;
const CHECK: &str = "  ✓ |";
const CROSS: &str = "  ⨯ |";

pub fn print_header(options: &Options) {
    let mut title = Vec::new();
    title.push("╭────────────────────────────────────────────────╮");
    title.push("│ Unlock and mount a LUKS partition              │");
    title.push("╰────────────────────────────────────────────────╯");
    let mut body = Vec::new();
    body.push(format!(
        "   Partition: {} ",
        options.partition_path.display()
    ));
    body.push(format!(
        " Mapper path: {}",
        options.get_mapper_path().display()
    ));
    body.push(format!("  Mount path: {}", options.mount_path.display()));
    body.push(format!(
        "    Key path: {}",
        options
            .key_path
            .clone()
            .map_or(String::new(), |path| path.display().to_string())
    ));
    eprintln!(
        "{}\n{}\n",
        title.join("\n").bold(),
        body.join("\n").dimmed()
    );
}

pub fn print_step_start(mut_counter: &Mutex<usize>, message: &str) {
    let mut i = mut_counter.lock().expect("Should be able to lock mutex");
    *i += 1;
    info!("{}", format!("{i}/{STEPS_COUNT} | {message}").dimmed());
}

pub fn print_step_completed(message: &str) {
    info!("{} {message}", CHECK.dimmed());
}

pub fn print_error(message: &str) {
    error!("{} {message}", CROSS.dimmed());
}
