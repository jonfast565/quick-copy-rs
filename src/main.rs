mod utilities;
mod paths;
mod files;
mod change_detector;
mod configuration;
mod copier;

fn main() {
    println!("Hello, world!");

    let program_options = configuration::ProgramOptions::new();
    let change_detector = change_detector::ChangeDetector::new(program_options);
    let copier = copier::Copier::new(program_options);
}
