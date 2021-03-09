#[derive(Clone)]
pub enum RuntimeType {
    Console,
    Service,
    Batch,
}

#[derive(Clone)]
pub struct ProgramOptions {
    pub runtime: RuntimeType,
    pub source_directory: String,
    pub target_directory: String,
    pub check_time: f64,
    pub enable_deletes: bool,
    pub skip_folders: Vec<String>,
    pub use_config_file: bool,
}

impl ProgramOptions {
    pub fn new_test() -> ProgramOptions {
        ProgramOptions {
            runtime: RuntimeType::Batch,
            source_directory: "C:\\Users\\jnfst\\Desktop\\Test1".to_string(),
            target_directory: "C:\\Users\\jnfst\\Desktop\\Test2".to_string(),
            check_time: 30000.00,
            enable_deletes: false,
            skip_folders: vec![],
            use_config_file: false,
        }
    }
}
