#![allow(dead_code)]

use chrono;

mod change_detector;
mod configuration;
mod copier;
mod files;
mod header;
mod paths;
mod utilities;
mod tests;

use log::{info};

fn main() {
    setup_logger().unwrap();
    info!("{}", header::get_header());

    let program_options = configuration::ProgramOptions::new_test();
    let change_detector = change_detector::ChangeDetector::new(program_options.clone());
    let copier = copier::Copier::new(program_options.clone());
    let actions = change_detector.incremental_changes();
    if actions.len() > 0 {
        copier.incremental_copy(actions);
    } else {
        info!("Nothing to do.")
    }

    info!("Done!");
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}