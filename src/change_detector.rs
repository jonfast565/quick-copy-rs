use crate::configuration::ProgramOptions;
use crate::files;
use crate::paths::ActionType;
use crate::paths::FileInfoParser;
use crate::paths::FileInfoParserAction;
use crate::paths::FileInfoParserActionList;
use crate::paths::PathParser;
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ChangeDetector {
    program_options: ProgramOptions,
}

impl ChangeDetector {
    pub fn new(o: ProgramOptions) -> ChangeDetector {
        ChangeDetector { program_options: o }
    }

    #[warn()]
    pub fn changed(&self) -> bool {
        info!("Checking for changes...");
        let merge = self.three_way_merge();
        merge.len() > 0
    }

    pub fn incremental_changes(&self) -> Vec<FileInfoParserActionList> {
        info!("Checking for changes...");
        let merge = self.three_way_merge();
        merge
    }

    pub fn three_way_merge(&self) -> Vec<FileInfoParserActionList> {
        info!("Merging...");

        let mut results: Vec<FileInfoParserActionList> = Vec::new();
        let source_dir = self.program_options.get_source_directory();
        info!("Source directory is {}", &source_dir);

        let target_dirs = self.program_options.get_target_directories();

        for target_dir in target_dirs {
            info!("Target directory is {}", target_dir);

            let source_pp = PathParser::new(&source_dir);
            let dest_pp = PathParser::new(&target_dir);

            if source_pp.get_segment().identical(&dest_pp.get_segment()) {
                error!(
                    "Source and destination paths are identical. 
                        Please change the paths to allow for copying."
                );
                continue;
            }

            info!("Trying to find the source directory...");
            let source_dir_path = Path::new(&source_dir);
            if !source_dir_path.exists() {
                warn!("Source dir doesn't exist; creating it.");
                fs::create_dir(source_dir_path).unwrap();
            } else {
                info!("Found.")
            }

            info!("Trying to find the target directory...");
            let target_dir_path = Path::new(&target_dir);
            if !target_dir_path.exists() {
                warn!("Target dir doesn't exist; creating it.");
                fs::create_dir(target_dir_path).unwrap();
            } else {
                info!("Found.")
            }

            info!("Enumerating the source directory...");
            let files1 = files::get_all_files(&source_dir).unwrap();
            let results1 = files1
                .iter()
                .map(|x| FileInfoParser::new(x, &source_dir))
                .collect::<Vec<FileInfoParser>>();
            info!("{} item(s) found in source.", &files1.len());

            info!("Enumerating the target directory...");
            let files2 = files::get_all_files(&target_dir).unwrap();
            let results2 = files2
                .iter()
                .map(|x| FileInfoParser::new(x, &target_dir))
                .collect::<Vec<FileInfoParser>>();
            info!("{} item(s) found in target.", &files2.len());

            info!("Building path caches...");
            let mut files1_hash = HashMap::<String, String>::new();
            for file1 in &results1 {
                files1_hash.insert(
                    file1
                        .get_segment()
                        .get_default_segment_string()
                        .to_lowercase(),
                    file1.get_path(),
                );
            }

            let mut files2_hash = HashMap::<String, String>::new();
            for file2 in &results2 {
                files2_hash.insert(
                    file2
                        .get_segment()
                        .get_default_segment_string()
                        .to_lowercase(),
                    file2.get_path(),
                );
            }

            let mut in_first_only = Vec::<FileInfoParser>::new();
            let mut in_both = Vec::<(FileInfoParser, FileInfoParser)>::new();

            info!("Checking for created or updated files...");
            for file1 in results1 {
                let key = file1
                    .get_segment()
                    .get_default_segment_string()
                    .to_lowercase();
                if files2_hash.contains_key(&key) {
                    let file2 = &files2_hash[&key];
                    let fif = FileInfoParser::new(&file2, &target_dir);
                    in_both.push((file1, fif));
                } else {
                    in_first_only.push(file1);
                }
            }
            info!("{} items to be created.", &in_first_only.len());
            info!("{} items to be updated.", &in_both.len());

            info!("Checking for deleted files...");
            let mut in_second_only = Vec::<FileInfoParser>::new();
            for file2 in results2 {
                let key = file2
                    .get_segment()
                    .get_default_segment_string()
                    .to_lowercase();
                if !files1_hash.contains_key(&key) {
                    in_second_only.push(file2);
                }
            }
            info!("{} items to be deleted.", &in_second_only.len());

            info!("Enumerating possible create actions...");
            let mut actions = Vec::<FileInfoParserAction>::new();
            let mut first_paths = in_first_only
                .clone()
                .iter()
                .map(|first| FileInfoParserAction::new_source(first.clone(), ActionType::Create))
                .collect::<Vec<FileInfoParserAction>>();

            info!("Enumerating possible delete actions...");
            let mut second_paths = in_second_only
                .clone()
                .iter()
                .map(|second| {
                    FileInfoParserAction::new_destination(second.clone(), ActionType::Delete)
                })
                .collect::<Vec<FileInfoParserAction>>();
            actions.append(&mut first_paths);
            actions.append(&mut second_paths);

            info!("Enumerating possible update actions...");
            let mut ignore_counter = 0;
            let mut use_counter = 0;
            let mut directory_counter = 0;
            for (first, second) in in_both {
                if !first.is_file || !second.is_file {
                    directory_counter += 1;
                    continue;
                }

                let first_modified = first.metadata.modified().unwrap();
                let second_modified = second.metadata.modified().unwrap();
                let first_len = first.metadata.len();
                let second_len = second.metadata.len();

                if first_len != second_len || first_modified != second_modified {
                    actions.push(FileInfoParserAction::new(first, second, ActionType::Update));
                    use_counter += 1;
                } else {
                    ignore_counter += 1;
                }
            }

            info!(
                "{} update actions on directories ignored.",
                directory_counter
            );
            info!(
                "{} update actions ignored based on file criteria.",
                ignore_counter
            );
            info!(
                "{} update actions used based on file criteria.",
                use_counter
            );
            info!("{} total actions found.", &actions.len());
            results.push(FileInfoParserActionList {
                actions: actions,
                source_directory: source_dir.clone(),
                target_directory: target_dir.clone(),
            })
        }

        results
    }
}
