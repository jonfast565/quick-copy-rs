use crate::configuration::ProgramOptions;
use crate::paths::{ActionType, FileInfoParserAction, PathParser};

use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

pub struct Copier {
    program_options: ProgramOptions,
}

impl Copier {
    pub fn new(o: ProgramOptions) -> Copier {
        Copier { program_options: o }
    }

    pub fn incremental_copy(&self, actions: Vec<FileInfoParserAction>) {
        let skip_folders = self
            .program_options
            .skip_folders
            .iter()
            .map(|x| PathParser::new(x))
            .collect::<Vec<PathParser>>();

        let ordered_creates = actions
            .clone()
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .filter(|x| x.action_type == ActionType::Create || x.action_type == ActionType::Update)
            .collect::<Vec<FileInfoParserAction>>();

        let ordered_deletes = actions
            .clone()
            .into_iter()
            .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .filter(|x| x.action_type == ActionType::Delete)
            .rev()
            .collect::<Vec<FileInfoParserAction>>();
        for c in ordered_creates {
            match c.action_type {
                ActionType::Create => {
                    let source = c.source.as_ref();
                    // let source_dir = self.program_options.source_directory.clone();
                    let dest_dir = self.program_options.target_directory.clone();
                    let destination_segment = c.get_destination_from_segment(&dest_dir);
                    for s in &skip_folders {
                        let skip_segment = s.segment.as_ref().unwrap().get_default_segment_string();
                        if source
                            .unwrap()
                            .segment
                            .as_ref()
                            .unwrap()
                            .contains_all_of_segment(&s.segment.as_ref().unwrap())
                        {
                            let source_path = &source.unwrap().get_path();
                            println!("Skipped {} because {} skipped.", source_path, skip_segment);
                        }
                    }

                    let src = source.unwrap().get_path();
                    let dst = destination_segment;

                    if c.source.unwrap().is_file {
                        //dbg!(&src);
                        //dbg!(&dst);
                        fs::copy(src, dst).unwrap();
                    } else {
                        fs::create_dir(dst).unwrap();
                    }
                }
                ActionType::Update => {
                    let source = c.source.as_ref();

                    for s in &skip_folders {
                        let skip_segment = s.segment.as_ref().unwrap().get_default_segment_string();
                        if source
                            .unwrap()
                            .segment
                            .as_ref()
                            .unwrap()
                            .contains_all_of_segment(&s.segment.as_ref().unwrap())
                        {
                            let source_path = source.unwrap().get_path();
                            println!("Skipped {} because {} skipped.", source_path, skip_segment);
                        }
                    }

                    let src = source.unwrap().get_path();
                    let dst = c.destination.unwrap().get_path();

                    if c.source.unwrap().is_file {
                        fs::copy(&src, dst).unwrap();
                        println!("Copied {} (changed)", src);
                    } else {
                        fs::create_dir(dst).unwrap();
                    }
                }
                ActionType::Delete => {
                    println!("Nothing to do.");
                }
            }
        }

        for d in ordered_deletes {
            match d.action_type {
                ActionType::Create => {
                    println!("Nothing to do.")
                }
                ActionType::Update => {
                    println!("Nothing to do.");
                }
                ActionType::Delete => {
                    if self.program_options.enable_deletes {
                        let destination = d.destination.as_ref();
                        let destination_path = destination.unwrap().get_path();
                        let file = destination.unwrap().is_file;
                        if file {
                            fs::remove_file(destination_path).unwrap();
                        } else {
                            fs::remove_dir(destination_path).unwrap();
                        }
                    } else {
                        println!("Deleted suppressed by config");
                        break;
                    }
                }
            }
        }
    }
}
