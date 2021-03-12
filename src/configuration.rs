use clap::{App, Arg};

#[derive(Clone)]
pub enum RuntimeType {
    Console,
    Service,
    Batch,
}

#[derive(Clone)]
pub struct ProgramOptions {
    pub runtime: RuntimeType,
    source_directory: String,
    target_directory: String,
    pub check_time: f64,
    pub enable_deletes: bool,
    pub skip_folders: Vec<String>,
    pub use_config_file: bool,
}

impl ProgramOptions {
    pub fn from_command_line_arguments() -> ProgramOptions {
        let app = App::new("QuickCopy")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Performs a quick and efficient incremental copy of files to a new directory.")
            .author("Jon Fast")
            .arg(
                Arg::with_name("source_directory")
                    .short("s")
                    .long("source")
                    .value_name("PATH")
                    .help("Sets the source directory for the copy")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("target_directory")
                    .short("t")
                    .long("target")
                    .value_name("PATH")
                    .help("Sets the target directory for the copy")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("d")
                    .short("d")
                    .multiple(false)
                    .required(false)
                    .help("Sets whether or not deletes will be enabled for the target directory"),
            )
            .arg(
                Arg::with_name("skip_folders")
                    .short("k")
                    .long("skip")
                    .value_name("PATH SEGMENT LIST")
                    .help("Sets the list of paths to skip when copying to the target directory")
                    .takes_value(true)
                    .multiple(true)
                    .required(false)
                    .value_delimiter(","),
            )
            .arg(
                Arg::with_name("runtime")
                    .short("r")
                    .long("runtime")
                    .value_name("TYPE")
                    .help("Sets the runtime mode")
                    .takes_value(true)
                    .default_value("Batch")
                    .required(true)
                    .possible_values(&["Console", "Batch"]),
            )
            .arg(
                Arg::with_name("check_interval")
                    .short("c")
                    .long("check")
                    .value_name("INTERVAL IN MS")
                    .help("Sets the interval to check for changes (Console Mode) in ms")
                    .default_value("3000")
                    .required(false),
            )
            .get_matches();
        ProgramOptions::new_test()
    }

    pub fn new_test() -> ProgramOptions {
        ProgramOptions {
            runtime: RuntimeType::Batch,
            source_directory:
                "C:\\Users\\jfast\\OneDrive - American College of Cardiology\\Desktop\\Test1"
                    .to_string(),
            target_directory:
                "C:\\Users\\jfast\\OneDrive - American College of Cardiology\\Desktop\\Test2"
                    .to_string(),
            check_time: 30000.00,
            enable_deletes: true,
            skip_folders: vec![],
            use_config_file: false,
        }
    }

    pub fn get_source_directory(&self) -> String {
        self.source_directory.clone()
    }

    pub fn get_target_directory(&self) -> String {
        self.target_directory.clone()
    }

    pub fn get_skip_folders(&self) -> Vec<String> {
        self.skip_folders.clone()
    }
}
