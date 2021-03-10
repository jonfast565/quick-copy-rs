#![allow(dead_code)]

mod change_detector;
mod configuration;
mod copier;
mod files;
mod header;
mod paths;
mod utilities;

fn main() {
    println!("{}", header::get_header());

    let program_options = configuration::ProgramOptions::new_test();
    let change_detector = change_detector::ChangeDetector::new(program_options.clone());
    let copier = copier::Copier::new(program_options.clone());
    let actions = change_detector.incremental_changes();
    
    if actions.len() > 0 {
        copier.incremental_copy(actions);
    } else {
        println!("Nothing to do.")
    }

    println!("Done!");
}
