use crate::paths::{PathParser, FileInfoParserAction, ActionType};
use crate::configuration::ProgramOptions;

use std::cmp::{Ordering};
use itertools::Itertools;

pub struct Copier {
    program_options: ProgramOptions,
}

impl Copier {
    pub fn new(o: ProgramOptions) -> Copier {
        Copier { program_options: o }
    }

    pub fn incremental_copy(&self, actions: Vec<FileInfoParserAction>) {
        let skip_folders = self.program_options.skip_folders.iter().map(|x| 
            PathParser::new(x.clone())).collect::<Vec<PathParser>>();
        
        let ordered_creates = actions.clone()
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .filter(|x| x.action_type == ActionType::Create || x.action_type == ActionType::Update)
            .collect::<Vec<FileInfoParserAction>>();

        let ordered_deletes = actions.clone()
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .filter(|x| x.action_type == ActionType::Delete)
            .rev()
            .collect::<Vec<FileInfoParserAction>>();
        
        for c in ordered_creates {
            for s in skip_folders {

            }
            match c.action_type {
                ActionType::Create => (),
                ActionType::Update => (),
                ActionType::Delete => ()
            }
        }

        for d in ordered_deletes {
            for s in skip_folders {
                
            }
            match d.action_type {
                ActionType::Create => (),
                ActionType::Update => (),
                ActionType::Delete => ()                
            }
        }
    }
}