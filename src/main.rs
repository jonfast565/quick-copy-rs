//#![allow(dead_code)]

#[macro_use]
extern crate clap;

use chrono;
use log::{error, info};
use std::{thread, time};

mod change_detector;
mod configuration;
mod copier;
mod files;
mod paths;
mod tests;
mod utilities;

use change_detector::ChangeDetector;
use configuration::{ProgramOptions, RuntimeType};
use copier::Copier;

fn main() {
    setup_logger().unwrap();
    let program_options = ProgramOptions::from_command_line_arguments();
    match &program_options.runtime {
        RuntimeType::Batch => run_batch_mode(program_options.clone()),
        RuntimeType::Console => run_console_mode(program_options.clone()),
        RuntimeType::Service => run_service_mode(program_options.clone()),
    }
    info!("Done!");
}

fn run_console_mode(o: ProgramOptions) {
    info!("Running in console mode");
    loop {
        run_cycle(o.clone());
        info!("Waiting {} ms", o.check_time);
        let ms = time::Duration::from_millis(o.check_time);
        thread::sleep(ms);
    }
}

fn run_batch_mode(o: ProgramOptions) {
    info!("Running in batch mode");
    run_cycle(o);
}

fn run_service_mode(_o: ProgramOptions) {
    info!("Running in service mode");
    error!("Not implemented as a Windows Service");
    panic!("Not implemented as a Windows Service");
}

fn run_cycle(o: ProgramOptions) {
    let change_detector = ChangeDetector::new(o.clone());
    let copier = Copier::new(o.clone());
    let actions = change_detector.incremental_changes();
    if actions.len() > 0 {
        copier.incremental_copy(actions);
    } else {
        info!("Nothing to do.")
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                // record.target(),
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
