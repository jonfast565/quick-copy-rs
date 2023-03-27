use crate::configuration::ProgramOptions;
use crate::paths::{ActionType, FileInfoParser, FileInfoParserAction, FileInfoParserActionList, PathParser};
use crate::utilities::{read_file_incremental_action};
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use xxhash_rust::xxh3::xxh3_64;

pub struct ChangeDetector {
    program_options: ProgramOptions,
}

impl ChangeDetector {
    pub fn new(o: ProgramOptions) -> ChangeDetector {
        ChangeDetector { program_options: o }
    }

    #[allow(dead_code)]
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
            locate_dir(&source_dir);

            info!("Trying to find the target directory...");
            locate_dir(&target_dir);

            let file_info_source = self.enumerate_directory(&source_dir, "source");
            let file_info_target = self.enumerate_directory(&target_dir, "target");

            info!("Building path caches...");
            let source_hash = build_file_hash_list(&file_info_source);
            let target_hash = build_file_hash_list(&file_info_target);

            let mut in_first_only = Vec::<FileInfoParser>::new();
            let mut in_both = Vec::<(FileInfoParser, FileInfoParser)>::new();

            check_created_updated_files(
                file_info_source,
                target_hash,
                &target_dir,
                &mut in_both,
                &mut in_first_only,
            );

            let in_second_only = check_deleted_files(file_info_target, source_hash);
            let actions = self.enumerate_actions(in_first_only, in_second_only, in_both);
            let actions_noskip = self.filter_skip_actions(actions);

            results.push(FileInfoParserActionList {
                actions: actions_noskip,
                source_directory: source_dir.clone(),
                target_directory: target_dir.clone(),
            })
        }

        results
    }

    fn enumerate_directory(&self, source_dir: &String, dir_type: &str) -> Vec<FileInfoParser> {
        info!("Enumerating the {} directory...", dir_type);
        let files1 = crate::files::get_all_files(source_dir).unwrap();
        let results1 = files1
            .iter()
            .map(|x| FileInfoParser::new(x, source_dir))
            .filter(|x| {
                x.match_extension(self.program_options.extensions.clone())
            })
            .collect::<Vec<FileInfoParser>>();
        info!("{} item(s) found in {}.", &files1.len(), dir_type);
        results1
    }

    fn filter_skip_actions(&self, actions: Vec<FileInfoParserAction>) -> Vec<FileInfoParserAction> {
        let skip_folders = self
            .program_options
            .get_skip_folders()
            .iter()
            .map(|x| PathParser::new(x))
            .collect::<Vec<PathParser>>();

        let mut actions_after_skipping = Vec::<FileInfoParserAction>::new();
        if skip_folders.is_empty() {
            return actions;
        }

        let mut delete_actions = actions
            .iter()
            .filter(|x| x.action_type == ActionType::Delete)
            .map(|x| x.clone())
            .collect::<Vec<FileInfoParserAction>>();

        let non_delete_actions = actions
            .iter()
            .filter(|x| x.action_type != ActionType::Delete)
            .map(|x| x.clone())
            .collect::<Vec<FileInfoParserAction>>();

        for action in non_delete_actions {
            for skip_folder in &skip_folders {
                let skip_segment = skip_folder.get_segment().get_default_segment_string();
                let action_source = action.source.clone().unwrap();
                let action_segment = action_source.get_segment();
                let skip_folder_segment = &skip_folder.get_segment();
                if action_segment.contains_all_of_segment(skip_folder_segment) {
                    let source_path = &action_source.get_path();
                    warn!(
                        "Skipped {} because path '{}' is skipped.",
                        source_path, skip_segment
                    );
                } else {
                    actions_after_skipping.push(action.clone());
                }
            }
        }

        actions_after_skipping.append(&mut delete_actions);
        actions_after_skipping
    }

    fn remap_update_actions(
        &self,
        in_both: Vec<(FileInfoParser, FileInfoParser)>,
        actions: &mut Vec<FileInfoParserAction>,
    ) {
        info!("Enumerating possible update actions...");
        let mut ignore_counter = 0;
        let mut use_counter = 0;
        let mut directory_counter = 0;
        for (first, second) in in_both {
            if !first.is_file || !second.is_file {
                directory_counter += 1;
                continue;
            }
    
            let mut include = false;

            if self.program_options.update_compare_size {
                let first_len = first.metadata.len();
                let second_len = second.metadata.len();
                include = include || (first_len != second_len);
            }

            if self.program_options.update_compare_modified {
                let first_modified = first.metadata.modified().unwrap();
                let second_modified = second.metadata.modified().unwrap();
                include = include || first_modified != second_modified
            }

            if self.program_options.update_compare_md5 {
                let first_hash = build_file_comparative_hash(&first);
                let second_hash = build_file_comparative_hash(&second);
                include = include || first_hash.trim() != second_hash.trim();
            }
    
            if include {
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
            "{} update actions ignored based on file change criteria.",
            ignore_counter
        );
        info!(
            "{} update actions used based on file criteria.",
            use_counter
        );
    }

    fn enumerate_actions(
        &self,
        in_first_only: Vec<FileInfoParser>,
        in_second_only: Vec<FileInfoParser>,
        in_both: Vec<(FileInfoParser, FileInfoParser)>,
    ) -> Vec<FileInfoParserAction> {
        let mut actions = Vec::<FileInfoParserAction>::new();

        let mut first_paths = remap_create_actions(in_first_only);
        let mut second_paths = remap_delete_actions(in_second_only);
    
        actions.append(&mut first_paths);
        actions.append(&mut second_paths);
    
        self.remap_update_actions(in_both, &mut actions);
    
        info!("{} total actions found.", &actions.len());
        actions
    }
}

fn remap_create_actions(in_first_only: Vec<FileInfoParser>) -> Vec<FileInfoParserAction> {
    info!("Enumerating possible create actions...");
    let first_paths = in_first_only
        .clone()
        .iter()
        .map(|first| FileInfoParserAction::new_source(first.clone(), ActionType::Create))
        .collect::<Vec<FileInfoParserAction>>();
    first_paths
}

fn remap_delete_actions(in_second_only: Vec<FileInfoParser>) -> Vec<FileInfoParserAction> {
    info!("Enumerating possible delete actions...");
    let second_paths = in_second_only
        .clone()
        .iter()
        .map(|second| FileInfoParserAction::new_destination(second.clone(), ActionType::Delete))
        .collect::<Vec<FileInfoParserAction>>();
    second_paths
}

fn check_deleted_files(
    file_info_target: Vec<FileInfoParser>,
    source_hash: HashMap<String, String>,
) -> Vec<FileInfoParser> {
    info!("Checking for deleted files...");
    let mut in_second_only = Vec::<FileInfoParser>::new();
    for file2 in file_info_target {
        let key = file2
            .get_segment()
            .get_default_segment_string()
            .to_lowercase();
        if !source_hash.contains_key(&key) {
            in_second_only.push(file2);
        }
    }
    info!("{} items to be deleted.", &in_second_only.len());
    in_second_only
}

fn check_created_updated_files(
    file_info_source: Vec<FileInfoParser>,
    target_hash: HashMap<String, String>,
    target_dir: &String,
    in_both: &mut Vec<(FileInfoParser, FileInfoParser)>,
    in_first_only: &mut Vec<FileInfoParser>,
) {
    info!("Checking for created or updated files...");
    for f in file_info_source {
        let key = f
            .get_segment()
            .get_default_segment_string()
            .to_lowercase();
        if target_hash.contains_key(&key) {
            let f2 = &target_hash[&key];
            let fif = FileInfoParser::new(&f2, target_dir);
            in_both.push((f, fif));
        } else {
            in_first_only.push(f);
        }
    }
    info!("{} items to be created.", &in_first_only.len());
    info!("{} items to be updated.", &in_both.len());
}

fn locate_dir(dir: &String) {
    let dir = Path::new(dir);
    if !dir.exists() {
        warn!("Source dir doesn't exist; creating it.");
        fs::create_dir(dir).unwrap();
    } else {
        info!("Found.")
    }
}

fn build_file_hash_list(file_info_list: &Vec<FileInfoParser>) -> HashMap<String, String> {
    let mut file_hash = HashMap::<String, String>::new();
    for file1 in file_info_list {
        file_hash.insert(
            file1
                .get_segment()
                .get_default_segment_string()
                .to_lowercase(),
            file1.get_path(),
        );
    }
    file_hash
}

fn build_file_comparative_hash(file_info: &FileInfoParser) -> String {
    let filename = file_info.get_path();
    let mut result_vec = Vec::new();
    let mut f = std::fs::File::open(filename).expect("Unable to open file");
    read_file_incremental_action(&mut f, | result: &[u8] | {
        let h = xxh3_64(result);
        result_vec.push(h);
    });
    let s_result = result_vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    s_result
}
