use crate::{paths::FileInfoParser, constants::{READ5192}};
use std::{fs::File, io::Read};

pub fn string_match(needle: String, haystack: String) -> bool {
    let needle_lower = needle.to_lowercase();
    let haystack_lower = haystack.to_lowercase();

    //dbg!(&needle_lower);
    //dbg!(&haystack_lower);

    if needle_lower.len() == 0 && haystack_lower.len() == 0 {
        return true;
    }

    for _ in needle_lower.matches(haystack_lower.as_str()) {
        return true;
    }

    false
}

pub fn string_match_str(needle: &str, haystack: &str) -> bool {
    string_match(String::from(needle), String::from(haystack))
}

#[allow(dead_code)]
pub fn char_match(needle: char, haystack: char) -> bool {
    let needle_lower = needle.to_lowercase().collect::<Vec<char>>();
    let haystack_lower = haystack.to_lowercase().collect::<Vec<char>>();

    haystack_lower[0] == needle_lower[0]
}

pub fn path_is_unc(path: &String) -> bool {
    path.starts_with("\\\\")
}

pub fn match_list_or_all(item: &String, items: Vec<String>) -> bool {
    if items.is_empty() {
        return true;
    }

    items.contains(item)
}


// TODO: Move into FileInfoParser object
pub fn match_finfo_parser_extension(
    finfo_parser: &FileInfoParser,
    extensions: Vec<String>,
) -> bool {
    if !finfo_parser.is_file {
        return true;
    }

    match finfo_parser.extension.as_ref() {
        Some(extension) => match_list_or_all(&extension, extensions),
        None => false,
    }
}

pub fn read_file_incremental_action<F: FnMut(&[u8])>(file: &mut File, mut do_something: F) {
    let mut buffer = [0; READ5192];
    //let mut count = 0;
    while let Ok(n) = file.read(&mut buffer[..]) {
        if n != READ5192 {
            let rest = &buffer[0..n];
            do_something(rest);
            break;
        } else {
            do_something(&buffer);
            //count += n;
        }
    }
}
