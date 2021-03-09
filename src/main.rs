mod utilities;
mod paths;
mod files;
mod change_detector;
mod configuration;
mod copier;

fn main() {
    println!("Hello, world!");

    let program_options = configuration::ProgramOptions::new_test();
    let change_detector = change_detector::ChangeDetector::new(program_options.clone());
    let copier = copier::Copier::new(program_options.clone());
    let actions = change_detector.incremental_changes();
    copier.incremental_copy(actions);

    println!("Done!");
}
