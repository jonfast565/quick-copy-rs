use crate::configuration::ProgramOptions;
use crate::files;
use crate::paths::ActionType;
use crate::paths::FileInfoParser;
use crate::paths::FileInfoParserAction;
use crate::paths::PathParser;
use crate::utilities;

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

        let source_dir = self.program_options.source_directory.clone();
        let target_dir = self.program_options.target_directory.clone();

        let source_pp = PathParser::new(&source_dir);
        let dest_pp = PathParser::new(&target_dir);

        //dbg!(&source_pp);
        //dbg!(&dest_pp);

        if source_pp
            .segment
            .unwrap()
            .identical(&dest_pp.segment.unwrap())
        {
            println!(
                "Source and destination paths are identical. 
                      Please change the paths to allow for copying."
            );
            return Vec::<FileInfoParserAction>::new();
        }

        let source_dir_path = Path::new(&source_dir);
        if !source_dir_path.exists() {
            println!("Source doesn't exist; creating it.");
            fs::create_dir(source_dir_path).unwrap();
        }

        let target_dir_path = Path::new(&target_dir);
        if !target_dir_path.exists() {
            println!("Directory doesn't exist; creating it.");
            fs::create_dir(target_dir_path).unwrap();
        }

        let files1 = files::visit_all(Path::new(&source_dir)).unwrap();
        let results1 = files1
            .iter()
            .map(|x| FileInfoParser::new(x, &source_dir));
        println!("{} item(s) found in source", &files1.len());

        let files2 = files::visit_all(Path::new(&target_dir)).unwrap();
        let results2 = files2
            .iter()
            .map(|x| FileInfoParser::new(x, &target_dir));
        println!("{} item(s) found in target", &files2.len());

        let mut in_first_only = Vec::<FileInfoParser>::new();
        let mut in_both = Vec::<(FileInfoParser, FileInfoParser)>::new();

        println!("Checking for created or updated files");
        for file1 in results1.clone() {
            let mut found_in_first_only = true;
            let files_in_both = results2.clone().filter(|file2| {
                utilities::string_match(
                    file1.segment.as_ref().unwrap().get_default_segment_string(),
                    file2.segment.as_ref().unwrap().get_default_segment_string(),
                )
            });

            for file2 in files_in_both {
                in_both.push((file1.clone(), file2));
                found_in_first_only = false;
            }

            if !found_in_first_only {
                continue;
            }

            in_first_only.push(file1);
        }

        println!("Checking for deleted files");
        let mut in_second_only = Vec::<FileInfoParser>::new();
        for file2 in results2 {
            let found_in_second_only = results1.clone().all(|file1| {
                !utilities::string_match(
                    file1.segment.as_ref().unwrap().get_default_segment_string(),
                    file2.segment.as_ref().unwrap().get_default_segment_string(),
                )
            });
            if found_in_second_only {
                in_second_only.push(file2);
            }
        }

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
