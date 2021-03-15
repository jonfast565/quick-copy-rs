use clap::{App, Arg};
use log::info;
use std::env;
use std::str::FromStr;

const HEADER: &'static str = r"
_____                       __      ____                                
/\  __`\          __        /\ \    /\  _`\                              
\ \ \/\ \  __  __/\_\    ___\ \ \/'\\ \ \/\_\    ___   _____   __  __    
\ \ \ \ \/\ \/\ \/\ \  /'___\ \ , < \ \ \/_/_  / __`\/\ '__`\/\ \/\ \   
 \ \ \\'\\ \ \_\ \ \ \/\ \__/\ \ \\`\\ \ \L\ \/\ \L\ \ \ \L\ \ \ \_\ \  
  \ \___\_\ \____/\ \_\ \____\\ \_\ \_\ \____/\ \____/\ \ ,__/\/`____ \ 
   \/__//_/\/___/  \/_/\/____/ \/_/\/_/\/___/  \/___/  \ \ \/  `/___/> \
                                                        \ \_\     /\___/
                                                         \/_/     \/__/ ";
const SEPARATOR: &'static str =
    r"----------------------------------------------------------------------";

pub fn get_header() -> String {
    let current_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    String::from(
        HEADER.to_owned()
            + "\n"
            + SEPARATOR
            + "\n"
            + "Version: "
            + crate_version!()
            + "\n"
            + "Author: "
            + crate_authors!("\n")
            + "\n"
            + "Working Directory: "
            + current_dir.as_str()
            + "\n"
            + SEPARATOR,
    )
}

#[derive(Clone)]
pub enum RuntimeType {
    Console,
    Service,
    Batch,
}

impl FromStr for RuntimeType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Console" => Ok(RuntimeType::Console),
            "Service" => Ok(RuntimeType::Service),
            "Batch" => Ok(RuntimeType::Batch),
            _ => Err("No match"),
        }
    }
}

#[derive(Clone)]
pub struct ProgramOptions {
    pub runtime: RuntimeType,
    source_directory: String,
    target_directories: Vec<String>,
    pub check_time: u64,
    pub enable_deletes: bool,
    pub skip_folders: Vec<String>,
    pub use_config_file: bool,
}

impl ProgramOptions {
    pub fn from_command_line_arguments() -> ProgramOptions {
        info!("{}", get_header());

        let app = App::new(crate_name!())
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!("\n"))
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
                Arg::with_name("target_directories")
                    .short("t")
                    .long("target")
                    .value_name("PATH LIST")
                    .help("Sets the target directories for the copy")
                    .takes_value(true)
                    .multiple(true)
                    .required(false)
                    .value_delimiter(","),
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

        let runtime: RuntimeType =
            value_t!(app.value_of("runtime"), RuntimeType).unwrap_or_else(|e| e.exit());
        let source_directory: String = if let Some(s) = app.value_of("source_directory") {
            s.to_string()
        } else {
            "".to_string()
        };
        let target_directories: Vec<String> = if let Some(s) = app.values_of("target_directories") {
            s.map(|x| x.to_string()).collect()
        } else {
            vec!()
        };
        let check_time: u64 = if let Some(s) = app.value_of("check_time") {
            s.parse::<u64>().unwrap()
        } else {
            3000
        };
        let enable_deletes: bool = match app.occurrences_of("d") {
            0 => false,
            1 => true,
            _ => false,
        };
        let skip_folders: Vec<String> = if let Some(s) = app.values_of("skip_folders") {
            s.map(|x| x.to_string()).collect()
        } else {
            vec!()
        };

        ProgramOptions {
            runtime: runtime,
            source_directory: source_directory,
            target_directories: target_directories,
            check_time: check_time,
            enable_deletes: enable_deletes,
            skip_folders: skip_folders,
            use_config_file: false,
        }
    }

    pub fn get_source_directory(&self) -> String {
        self.source_directory.clone()
    }

    pub fn get_target_directories(&self) -> Vec<String> {
        self.target_directories.clone()
    }

    pub fn get_skip_folders(&self) -> Vec<String> {
        self.skip_folders.clone()
    }
}
