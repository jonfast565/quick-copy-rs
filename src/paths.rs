use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use log::debug;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::utilities;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Create,
    Update,
    Delete,
}

#[derive(Clone, Debug)]
pub enum MatchType<First, Second> {
    Match(First, Second),
    NoMatch(First, Second),
    OnlyOneLeft(First),
    OnlyOneRight(Second),
}

pub const WINDOWS_SPLITTER: char = '\\';
pub const UNIX_SPLITTER: char = '/';
pub const SPLITTER: char = '|';

#[derive(Clone, Debug)]
pub struct PathSegment {
    name: String,
    next: Option<Box<PathSegment>>,
}

impl PathSegment {
    pub fn get_remaining_segments(&self) -> Vec<PathSegment> {
        let current_segment = self;
        let mut results = Vec::<PathSegment>::new();
        results.push(current_segment.clone());
        let mut seg_option = current_segment.next.as_ref();
        while let Some(i) = seg_option {
            results.push(*i.clone());
            seg_option = match &seg_option.unwrap().next {
                Some(x) => Some(&x),
                None => None,
            };
        }
        results
    }

    pub fn get_segment_string(&self, separator: char) -> String {
        let remaining_segments = self.get_remaining_segments();
        let mut segment_string = String::new();
        for seg in remaining_segments {
            segment_string.push_str(seg.name.as_str());
            segment_string.push(separator);
        }
        segment_string.pop();
        segment_string
    }

    pub fn get_segments(&self) -> Vec<String> {
        let remaining_segments = self.get_remaining_segments();
        let string_vec: Vec<String> = remaining_segments.iter().map(|x| x.name.clone()).collect();
        string_vec
    }

    pub fn get_default_segment_string(&self) -> String {
        if cfg!(windows) {
            self.get_segment_string(WINDOWS_SPLITTER)
        } else if cfg!(unix) {
            self.get_segment_string(UNIX_SPLITTER)
        } else {
            self.get_segment_string(UNIX_SPLITTER)
        }
    }

    pub fn get_segment_length(&self) -> usize {
        let segment_string = self.get_default_segment_string();
        let splitted: Vec<&str> = segment_string.split(SPLITTER).collect();
        splitted.len()
    }

    pub fn contains_all_of_segment(&self, folder_segment: &PathSegment) -> bool {
        let str1 = self.get_segment_string(SPLITTER);
        let str2 = folder_segment.get_segment_string(SPLITTER);
        let split1 = str1.split(SPLITTER).collect::<Vec<&str>>();
        let split2 = str2.split(SPLITTER).collect::<Vec<&str>>();
        let mut split_ctr = 0;

        for t in split1 {
            if utilities::string_match_str(split2[split_ctr], t) {
                split_ctr += 1;

                if split_ctr == split2.len() {
                    return true;
                }
            } else {
                split_ctr = 0;
            }
        }

        false
    }

    pub fn identical(&self, other_segment: &PathSegment) -> bool {
        let str1 = self.get_segment_string(SPLITTER);
        let str2 = other_segment.get_segment_string(SPLITTER);
        let split1 = str1.split(SPLITTER).collect::<Vec<&str>>();
        let split2 = str2.split(SPLITTER).collect::<Vec<&str>>();

        if split1.len() != split2.len() {
            return false;
        }

        for i in 0..split1.len() {
            if !utilities::string_match_str(split1[i], split2[i]) {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct PathParser {
    segment: Option<Box<PathSegment>>,
}

impl PathParser {
    pub fn new(path: &String) -> PathParser {
        PathParser::build_segments(&path)
    }

    fn normalized_splitter(path: &String) -> Vec<String> {
        let fixed_string = path
            .replace(WINDOWS_SPLITTER, SPLITTER.to_string().as_str())
            .replace(UNIX_SPLITTER, SPLITTER.to_string().as_str());
        let normalized = fixed_string
            .split(SPLITTER)
            .filter(|x| !x.is_empty() || x != &"?")
            .map(String::from)
            .collect::<Vec<String>>();
        normalized
    }

    fn build_segments(path: &String) -> PathParser {
        let mut normalized = PathParser::normalized_splitter(&path);
        normalized.reverse();
        let mut next_segment: Option<Box<PathSegment>> = None;

        debug!("Build segments for: {}.", &path);

        for seg in normalized {
            let new_item = PathSegment {
                name: seg,
                next: next_segment,
            };
            next_segment = Some(Box::new(new_item));
        }

        PathParser {
            segment: next_segment,
        }
    }

    pub fn get_differing_segment(&self, p: PathParser) -> Option<Box<PathSegment>> {
        let other_segment_list = p.segment.as_ref().unwrap().get_remaining_segments();
        let my_segments = self.segment.as_ref().unwrap().get_remaining_segments();

        //dbg!(&my_segments);
        //dbg!(&other_segment_list);

        let zipped = my_segments.iter().zip_longest(other_segment_list);
        //dbg!(&zipped);
        for zip in zipped {
            let zip_val = match zip {
                Left(x) => MatchType::OnlyOneLeft(x.clone()),
                Right(x) => MatchType::OnlyOneRight(x.clone()),
                Both(x, y) => {
                    if utilities::string_match_str(&x.name, &y.name) {
                        MatchType::Match(x.clone(), y.clone())
                    } else {
                        MatchType::NoMatch(x.clone(), y.clone())
                    }
                }
            };

            if let MatchType::OnlyOneRight(x) = &zip_val {
                return Some(Box::new(x.clone()));
            }

            if let MatchType::NoMatch(_, y) = &zip_val {
                return Some(Box::new(y.clone()));
            }
        }

        None
    }

    pub fn append_segment(&self, new_segment: &String) -> PathParser {
        let segment_pp = PathParser::new(new_segment);
        let mut segs_pp_arr = segment_pp.segment.unwrap().get_segments();
        let my_segment_string = self.segment.as_ref().unwrap().get_default_segment_string();
        let my_pp = PathParser::new(&my_segment_string);
        let mut my_pp_arr = my_pp.segment.unwrap().get_segments();
        let mut new_arr: Vec<String> = Vec::new();
        new_arr.append(&mut my_pp_arr);
        new_arr.append(&mut segs_pp_arr);
        let formatted_sep = format!("{}", SPLITTER);
        let new_path = new_arr.join(&formatted_sep);
        let final_pp = PathParser::new(&new_path);
        final_pp
    }

    #[allow(dead_code)]
    fn get_last(&self) -> Option<PathSegment> {
        let initial_segment = self.segment.clone();
        let mut segment = initial_segment.clone();

        let mut queue = VecDeque::<PathSegment>::new();
        while let Some(ref x) = segment {
            queue.push_back(*x.clone());
            segment = match segment.unwrap().next {
                Some(x) => Some(x),
                None => None,
            };
        }

        if queue.len() > 0 {
            let last = queue.pop_back();
            return last;
        }

        None
    }

    #[allow(dead_code)]
    fn remove_last(&self) -> Option<PathSegment> {
        let initial_segment = self.segment.clone();
        let mut segment = initial_segment.clone();

        let mut queue = VecDeque::<PathSegment>::new();
        while let Some(ref x) = segment {
            queue.push_back(*x.clone());
            segment = Some(segment.unwrap().next.unwrap());
        }

        if queue.len() > 0 {
            let last = queue.pop_back();
            return last;
        }

        Some(*initial_segment.unwrap())
    }

    pub fn get_segment(&self) -> Box<PathSegment> {
        let unwrapped_segment = self.segment.as_ref().unwrap();
        unwrapped_segment.clone()
    }
}

#[derive(Clone, Debug)]
pub struct FileInfoParser {
    segment: Option<Box<PathSegment>>,
    pub metadata: fs::Metadata,
    pub is_file: bool,
    pub is_unc_path: bool,
    pub path: String,
    pub extension: Option<String>,
    pub filename: Option<String>,
}

impl FileInfoParser {
    pub fn new(path: &String, base_directory: &String) -> FileInfoParser {
        let md = fs::metadata(&path).unwrap();
        let base_parser = PathParser::new(&base_directory);
        let path_buf = Path::new(&path);
        let mut extension = path_buf
            .extension()
            .and_then(OsStr::to_str)
            .and_then(|x| Some(x.to_string()));
        let path_string = path_buf.as_os_str().to_str().unwrap().to_string();
        let sub_dir_parser = PathParser::new(&path_string);
        let seg = base_parser.get_differing_segment(sub_dir_parser);
        let filename = path_buf
            .file_name()
            .and_then(OsStr::to_str)
            .and_then(|x| Some(x.to_string()));

        let new_filename = match filename {
            Some(f) => {
                if f.starts_with(".") {
                    let temp_extension = f.replace(".", "").to_string();
                    extension = Some(temp_extension);
                    None
                } else {
                    Some(f)
                }
            }
            _ => None,
        };

        let result = FileInfoParser {
            is_file: !md.is_dir(),
            metadata: md,
            segment: seg,
            is_unc_path: utilities::path_is_unc(base_directory),
            path: path.to_string(),
            extension: extension,
            filename: new_filename,
        };

        result
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_segment(&self) -> Box<PathSegment> {
        let unwrapped_segment = self.segment.as_ref().unwrap();
        unwrapped_segment.clone()
    }

    pub fn match_extension(&self, extensions: Vec<String>) -> bool {
        if !self.is_file {
            return true;
        }

        match self.extension.as_ref() {
            Some(extension) => utilities::match_list_or_all(&extension, extensions),
            None => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileInfoParserAction {
    pub source: Option<FileInfoParser>,
    pub destination: Option<FileInfoParser>,
    pub action_type: ActionType,
}

impl PartialEq for FileInfoParserAction {
    fn eq(&self, other: &Self) -> bool {
        self.source.as_ref().unwrap().path == other.source.as_ref().unwrap().path
            && self.destination.as_ref().unwrap().path == other.destination.as_ref().unwrap().path
    }
}

impl PartialOrd for FileInfoParserAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_seg = match self.source.as_ref() {
            Some(x) => x,
            None => self.destination.as_ref().unwrap(),
        };

        let other_seg = match other.source.as_ref() {
            Some(x) => x,
            None => other.destination.as_ref().unwrap(),
        };

        let self_seg_len = self_seg.segment.as_ref().unwrap().get_segment_length();

        let other_seg_len = other_seg.segment.as_ref().unwrap().get_segment_length();
        if self_seg_len > other_seg_len {
            return Some(Ordering::Greater);
        } else if self_seg_len < other_seg_len {
            return Some(Ordering::Less);
        } else if self_seg_len == other_seg_len {
            return Some(Ordering::Equal);
        } else {
            return None;
        }
    }
}

impl FileInfoParserAction {
    pub fn new(
        source: FileInfoParser,
        dest: FileInfoParser,
        t: ActionType,
    ) -> FileInfoParserAction {
        FileInfoParserAction {
            source: Some(source),
            destination: Some(dest),
            action_type: t,
        }
    }

    pub fn new_source(source: FileInfoParser, t: ActionType) -> FileInfoParserAction {
        FileInfoParserAction {
            source: Some(source),
            destination: None,
            action_type: t,
        }
    }

    pub fn new_destination(dest: FileInfoParser, t: ActionType) -> FileInfoParserAction {
        FileInfoParserAction {
            source: None,
            destination: Some(dest),
            action_type: t,
        }
    }

    #[allow(dead_code)]
    pub fn get_source_length(&self) -> usize {
        self.source.clone().unwrap().path.len()
    }

    #[allow(dead_code)]
    pub fn get_destination_length(&self) -> usize {
        self.destination.clone().unwrap().path.len()
    }

    pub fn get_destination_from_segment(&self, target_directory: &String) -> String {
        let mut pp = PathParser::new(target_directory);
        let segment_string = self
            .source
            .clone()
            .unwrap()
            .segment
            .as_ref()
            .unwrap()
            .get_default_segment_string();
        pp = pp.append_segment(&segment_string);
        let destination_segment = pp.segment.unwrap().get_default_segment_string();
        destination_segment
    }
}

pub struct FileInfoParserActionList {
    pub source_directory: String,
    pub target_directory: String,
    pub actions: Vec<FileInfoParserAction>,
}
