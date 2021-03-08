pub enum RuntimeType {
    Console,
    Service,
    Batch
}

pub struct ProgramOptions {
    pub runtime: RuntimeType,
    pub source_directory: String,
    pub target_directory: String,
    pub check_time: f64,
    pub enable_deletes: bool,
    pub skip_folders: Vec<String>,
    pub use_config_file: bool
}