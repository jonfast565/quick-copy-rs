use crate::paths::PathParser;
use crate::paths::FileInfoParserAction;

pub struct Copier {
    program_options: ProgramOptions,
}

impl Copier {
    pub fn new(o: ProgramOptions) -> ChangeDetector {
        Copier { program_options: o }
    }

    pub fn incremental_copy(&self, actions: Vec<FileInfoParserAction>) {
        let skip_folders = self.program_options.skip_folders.map(|x| 
            PathParser::new(x)).collect::<Vec<PathParser>>();
        
        let ordered_creates = actions.clone().iter().filter(|x| x.action_type == ActionType::Create || x.action_type == ActionType::Update)
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .rev()
            .collect::<Vec<FileInfoParserAction>>();

        let ordered_deletes = actions.clone().iter().filter(|x| x.action_type == ActionType::Delete)
            .sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .rev()
            .collect::<Vec<FileInfoParserAction>>();
            
    }
}