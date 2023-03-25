use clap::{Parser, ValueEnum};

use std::env;
use std::fmt::Display;
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

#[derive(ValueEnum, Clone, Debug)]
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

impl Display for RuntimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            RuntimeType::Console => "console",
            RuntimeType::Service => "service",
            RuntimeType::Batch => "batch",
        };
        write!(f, "{}", value)
    }
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about)]
pub struct ProgramOptions {
    #[arg(long, value_name = "runtime-type", default_value_t = RuntimeType::Batch)]
    pub runtime: RuntimeType,

    #[arg(short = 's', long, value_name = "source-dir")]
    source_directory: String,

    #[arg(short = 't', long, value_name = "target-dirs")]
    target_directories: Vec<String>,

    #[arg(long, value_name = "check-time", default_value_t=20000)]
    pub check_time: u64,

    #[arg(short = 'e', long, value_name = "enable-deletes")]
    pub enable_deletes: bool,

    #[arg(long, value_name = "skip-folders")]
    pub skip_folders: Vec<String>,

    #[arg(short = 'x', long, value_name = "extensions")]
    pub extensions: Vec<String>,

    #[arg(long, value_name = "use-config-file")]
    pub use_config_file: bool,

    #[arg(long, value_name = "compare-modified")]
    pub update_compare_modified: bool,

    #[arg(long, value_name = "compare-size")]
    pub update_compare_size: bool,

    #[arg(long, value_name = "compare-md5")]
    pub update_compare_md5: bool,
}

impl ProgramOptions {
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
