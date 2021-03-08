use crate::configuration::ProgramOptions;
use crate::files;
use crate::paths::FileInfoParser;
use crate::paths::FileInfoParserAction;
use crate::paths::PathParser;

use std::fs;
use std::path::Path;

pub struct ChangeDetector {
    program_options: ProgramOptions,
}

impl ChangeDetector {
    pub fn new(o: ProgramOptions) -> ChangeDetector {
        ChangeDetector { program_options: o }
    }

    pub fn changed(&self) -> bool {
        println!("Checking for changes...");
        let merge = self.three_way_merge();
        merge.len() > 0
    }

    pub fn incremental_changes(&self) -> Vec<FileInfoParserAction> {
        println!("Checking for changes...");
        Vec::<FileInfoParserAction>::new()
    }

    pub fn three_way_merge(&self) -> Vec<FileInfoParserAction> {
        println!("Merging...");

        let source_dir = self.program_options.source_directory.clone();
        let target_dir = self.program_options.target_directory.clone();

        let source_pp = PathParser::new(source_dir.clone());
        let dest_pp = PathParser::new(target_dir.clone());

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

        let target_dir_path = Path::new(&target_dir);
        if !target_dir_path.exists() {
            println!("Directory doesn't exist; creating it.");
            fs::create_dir(target_dir_path).unwrap();
        }

        let files1 = files::enumerate_files(&source_dir).unwrap();
        let results1 = files1
            .iter()
            .map(|x| FileInfoParser::new(x.to_string(), source_dir.clone()));

        println!("{} item(s) found in source", &files1.len());

        let files2 = files::enumerate_files(&target_dir).unwrap();
        let results2 = files2
            .iter()
            .map(|x| FileInfoParser::new(x.to_string(), target_dir.clone()));

        println!("{} item(s) found in target", &files2.len());

        let in_first_only = Vec::<FileInfoParser>::new();
        let in_both = Vec::<FileInfoParser>::new();

        println!("Checking for created or updated files");

        for file1 in results1 {
            let mut found_in_first_only = true;
        }

        Vec::<FileInfoParserAction>::new()
    }
}
