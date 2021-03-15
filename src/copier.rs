use crate::configuration::ProgramOptions;
use crate::paths::{ActionType, FileInfoParserAction, PathParser, FileInfoParserActionList};

use itertools::Itertools;
use log::{info, warn};
use std::cmp::Ordering;
use std::fs;

pub struct Copier {
    program_options: ProgramOptions,
}

impl Copier {
    pub fn new(o: ProgramOptions) -> Copier {
        Copier { program_options: o }
    }

    pub fn incremental_copy(&self, action_list: Vec<FileInfoParserActionList>) {
        for action_item in action_list {
            let actions = action_item.actions;
            let skip_folders = self
                .program_options
                .get_skip_folders()
                .iter()
                .map(|x| PathParser::new(x))
                .collect::<Vec<PathParser>>();

            let ordered_creates = actions
                .clone()
                .into_iter()
                .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .filter(|x| {
                    x.action_type == ActionType::Create || x.action_type == ActionType::Update
                })
                .collect::<Vec<FileInfoParserAction>>();

            let ordered_deletes = actions
                .clone()
                .into_iter()
                .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .filter(|x| x.action_type == ActionType::Delete)
                .rev()
                .collect::<Vec<FileInfoParserAction>>();

            let mut counter = 0;
            let total = ordered_creates.len() + ordered_deletes.len();

            for c in ordered_creates {
                match c.action_type {
                    ActionType::Create => {
                        let source = c.source.as_ref();
                        // let source_dir = self.program_options.source_directory.clone();
                        let dest_dir = action_item.target_directory.clone();
                        let destination_segment = c.get_destination_from_segment(&dest_dir);
                        for s in &skip_folders {
                            let skip_segment = s.get_segment().get_default_segment_string();
                            if source
                                .unwrap()
                                .get_segment()
                                .contains_all_of_segment(&s.get_segment())
                            {
                                let source_path = &source.unwrap().get_path();
                                warn!("Skipped {} because {} skipped.", source_path, skip_segment);
                            }
                        }

                        let src = source.unwrap().get_path();
                        let dst = destination_segment;

                        if c.source.unwrap().is_file {
                            info!("Copying {} to {}", &src, &dst);
                            fs::copy(src, dst).unwrap();
                        } else {
                            info!("Creating dir {}", &dst);
                            fs::create_dir(dst).unwrap();
                        }
                    }
                    ActionType::Update => {
                        let source = c.source.as_ref();

                        for s in &skip_folders {
                            let skip_segment = s.get_segment().get_default_segment_string();
                            if source
                                .unwrap()
                                .get_segment()
                                .contains_all_of_segment(&s.get_segment())
                            {
                                let source_path = source.unwrap().get_path();
                                info!("Skipped {} because {} skipped.", source_path, skip_segment);
                            }
                        }

                        let src = source.unwrap().get_path();
                        let dst = c.destination.unwrap().get_path();

                        if c.source.unwrap().is_file {
                            info!("Copying {} to {}", &src, &dst);
                            fs::copy(&src, dst).unwrap();
                        } else {
                            info!("Creating dir {}", &dst);
                            fs::create_dir(dst).unwrap();
                        }
                    }
                    ActionType::Delete => {
                        info!("Nothing to do.");
                    }
                }
                counter += 1;
                info!(
                    "{} / {} operations performed ({}%).",
                    counter,
                    total,
                    ((counter as f64 / total as f64) * 100.0).round() as i64
                );
            }

            for d in ordered_deletes {
                match d.action_type {
                    ActionType::Create => {
                        info!("Nothing to do.")
                    }
                    ActionType::Update => {
                        info!("Nothing to do.");
                    }
                    ActionType::Delete => {
                        if self.program_options.enable_deletes {
                            let destination = d.destination.as_ref();
                            let destination_path = destination.unwrap().get_path();
                            let file = destination.unwrap().is_file;
                            if file {
                                info!("Remove file {}", &destination_path);
                                fs::remove_file(destination_path).unwrap();
                            } else {
                                info!("Remove directory {}", &destination_path);
                                fs::remove_dir(destination_path).unwrap();
                            }
                        } else {
                            info!("Deleted suppressed by config");
                            break;
                        }
                    }
                }
                counter += 1;
                info!(
                    "{} / {} operations performed ({}%).",
                    counter,
                    total,
                    ((counter as f64 / total as f64) * 100.0).round() as i64
                );
            }
        }
        info!("Copy operations completed");
    }
}
