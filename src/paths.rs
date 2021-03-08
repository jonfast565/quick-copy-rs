use std::collections::VecDeque;
use itertools::Itertools;
use itertools::EitherOrBoth::{Left, Right, Both};

use crate::utilities;

pub enum ActionType {
    Create,
    Update,
    Delete
}

pub enum MatchType<First, Second> {
    Match(First, Second),
    NoMatch(First, Second),
    OnlyOneLeft(First),
    OnlyOneRight(Second)
}

pub const WINDOWS_SPLITTER: char = '\\';
pub const UNIX_SPLITTER: char = '/';
pub const SPLITTER: char = '|';

#[derive(Clone)]
pub struct PathSegment {
    name: String,
    next: Option<Box<PathSegment>>
}

impl PathSegment {
    fn get_remaining_segments(&self) -> Vec<PathSegment> {
        let current_segment = self;
        let mut results = Vec::<PathSegment>::new();
        results.push(current_segment.clone());
        let mut seg_option = current_segment.next.as_ref();
        while let Some(i) = seg_option {
            results.push(*i.clone());
            seg_option = i.next.as_ref();
        }
        results
    }

    fn get_segment_string(&self, separator: char) -> String {
        let remaining_segments = self.get_remaining_segments();
        let mut segment_string = String::new();
        for seg in remaining_segments {
            segment_string.push_str(seg.name.as_str());
            segment_string.push(separator);
        }
        segment_string.pop();
        segment_string
    }

    fn get_default_segment_string(&self) -> String {
        if cfg!(windows) {
            self.get_segment_string(WINDOWS_SPLITTER)
        } else if cfg!(unix) {
            self.get_segment_string(UNIX_SPLITTER)
        } else {
            self.get_segment_string(UNIX_SPLITTER)
        }
    }

    fn get_segment_length(&self) -> usize {
        let segment_string = self.get_default_segment_string();
        let splitted : Vec<&str> = segment_string
            .split(SPLITTER)
            .collect();
        splitted.len()
    }

    fn contains_all_of_segment(&self, folder_segment: &PathSegment) -> bool {
        let str1 = self.get_segment_string(SPLITTER);
        let str2 = folder_segment.get_segment_string(SPLITTER);
        let split1 = str1.split(SPLITTER).collect::<Vec<&str>>();
        let split2 = str2.split(SPLITTER).collect::<Vec<&str>>();
        let mut split_ctr = 0;

        for t in split1 {
            if utilities::string_match_str(split2[split_ctr], t) {
                split_ctr += 1;

                if split_ctr == split2.len() {
                    return true
                }
            } else {
                split_ctr = 0;
            }
        }

        false
    }

    fn identical(&self, other_segment: &PathSegment) -> bool {
        let str1 = self.get_segment_string(SPLITTER);
        let str2 = other_segment.get_segment_string(SPLITTER);
        let split1 = str1.split(SPLITTER).collect::<Vec<&str>>();
        let split2 = str2.split(SPLITTER).collect::<Vec<&str>>();

        if split1.len() != split2.len() {
            return false;
        }

        for i in 0..split1.len() {
            if !utilities::string_match_str(split1[i], split2[i]) {
                return false
            }
        }
        
        true
    }
}

fn normalize_path(path: String) -> String {
    let normalized = path
    .replace(WINDOWS_SPLITTER, SPLITTER.to_string().as_str())
    .replace(UNIX_SPLITTER, SPLITTER.to_string().as_str());
    normalized
}

pub struct PathParser {
    segment: Option<Box<PathSegment>>
}

impl PathParser {
    fn new(path: String) -> PathParser {
        PathParser::build_segments(path)
    }

    fn build_segments(path: String) -> PathParser {
        let mut normalized = normalize_path(path)
            .split(SPLITTER)
            .map(String::from)
            .collect::<Vec<String>>();
        normalized.reverse();

        let mut next_segment : Option<Box<PathSegment>> = None;

        for seg in normalized {
            let new_item = PathSegment {
                name: seg,
                next: next_segment
            };
            next_segment = Some(Box::new(new_item));
        }

        PathParser {
            segment: next_segment
        }
    }

    pub fn get_differing_segment(&self, p : PathParser) -> Option<Box<PathSegment>> {
        let other_segment_list = p.segment.as_ref().unwrap().get_remaining_segments();
        let my_segments = self.segment.as_ref().unwrap().get_remaining_segments();

        let zipped = my_segments.iter().zip_longest(other_segment_list);

        for zip in zipped {
            let zip_val = match zip {
                Left(x) => MatchType::OnlyOneLeft(x.clone()),
                Right(x) => MatchType::OnlyOneRight(x.clone()),
                Both(x,y) => {
                    if utilities::string_match_str(&x.name, &y.name) {
                        MatchType::Match(x.clone(), y.clone());
                    }
                    MatchType::NoMatch(x.clone(), y.clone())
                }
            };

            if let MatchType::OnlyOneRight(x) = &zip_val { 
                return Some(x.next.clone().unwrap());
            }

            if let MatchType::NoMatch(_, y) = &zip_val {
                return Some(y.next.clone().unwrap());
            }
        }

        None
    }

    pub fn append_segment(&self, new_segment: String) -> &PathParser {
        let segment_parser = PathParser::new(new_segment);
        let last = self.get_last();
        last.unwrap().next = segment_parser.segment;
        self
    }
    
    fn get_last(&self) -> Option<PathSegment> {
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

        None
    }

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
}