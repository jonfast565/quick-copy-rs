use crate::configuration::ProgramOptions;
use crate::files;
use crate::paths::ActionType;
use crate::paths::FileInfoParser;
use crate::paths::FileInfoParserAction;
use crate::paths::PathParser;
use crate::utilities;

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
        println!("Checking for changes...");
        let merge = self.three_way_merge();
        merge.len() > 0
    }

    pub fn incremental_changes(&self) -> Vec<FileInfoParserAction> {
        println!("Checking for changes...");
        let merge = self.three_way_merge();
        merge
    }

    pub fn three_way_merge(&self) -> Vec<FileInfoParserAction> {
        println!("Merging...");

        let source_dir = self.program_options.get_source_directory();
        let target_dir = self.program_options.get_target_directory();

        let source_pp = PathParser::new(&source_dir);
        let dest_pp = PathParser::new(&target_dir);

        //dbg!(&source_pp);
        //dbg!(&dest_pp);

        if source_pp.get_segment().identical(&dest_pp.get_segment()) {
            println!(
                "Source and destination paths are identical. 
                      Please change the paths to allow for copying."
            );
            return Vec::<FileInfoParserAction>::new();
        }

        let source_dir_path = Path::new(&source_dir);
        if !source_dir_path.exists() {
            println!("Source dir doesn't exist; creating it.");
            fs::create_dir(source_dir_path).unwrap();
        }

        let target_dir_path = Path::new(&target_dir);
        if !target_dir_path.exists() {
            println!("Target dir doesn't exist; creating it.");
            fs::create_dir(target_dir_path).unwrap();
        }

        let files1 = files::get_all_files(&source_dir).unwrap();
        let results1 = files1
            .iter()
            .map(|x| FileInfoParser::new(x, &source_dir))
            .collect::<Vec<FileInfoParser>>();
        println!("{} item(s) found in source", &files1.len());

        let files2 = files::get_all_files(&target_dir).unwrap();
        let results2 = files2
            .iter()
            .map(|x| FileInfoParser::new(x, &target_dir))
            .collect::<Vec<FileInfoParser>>();
        println!("{} item(s) found in target", &files2.len());

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

        // TODO: Use hashes to improve performance
        let mut in_first_only = Vec::<FileInfoParser>::new();
        let mut in_both = Vec::<(FileInfoParser, FileInfoParser)>::new();

        println!("Checking for created or updated files");
        for file1 in results1 {
            let key = file1.get_segment().get_default_segment_string().to_lowercase();
            if files2_hash.contains_key(&key) {
                let file2 = &files2_hash[&key];
                let fif = FileInfoParser::new(&file2, &target_dir);
                in_both.push((file1, fif));
            } else {
                in_first_only.push(file1);
            }
        }
        println!("{} items to be created", &in_first_only.len());
        println!("{} items to be updated", &in_both.len());

        println!("Checking for deleted files");
        let mut in_second_only = Vec::<FileInfoParser>::new();
        for file2 in results2 {
            let key = file2.get_segment().get_default_segment_string().to_lowercase();
            if !files1_hash.contains_key(&key) {
                in_second_only.push(file2);
            }
        }
        println!("{} items to be deleted", &in_second_only.len());

        println!("Enumerating possible actions");
        let mut actions = Vec::<FileInfoParserAction>::new();
        let mut first_paths = in_first_only
            .clone()
            .iter()
            .map(|first| FileInfoParserAction::new_source(first.clone(), ActionType::Create))
            .collect::<Vec<FileInfoParserAction>>();
        let mut second_paths = in_second_only
            .clone()
            .iter()
            .map(|second| FileInfoParserAction::new_destination(second.clone(), ActionType::Delete))
            .collect::<Vec<FileInfoParserAction>>();
        actions.append(&mut first_paths);
        actions.append(&mut second_paths);

        for (first, second) in in_both {
            if !first.is_file || !second.is_file {
                continue;
            }

            if first.metadata.len() != second.metadata.len() {
                actions.push(FileInfoParserAction::new(first, second, ActionType::Update));
            }
        }
        actions
    }
}
