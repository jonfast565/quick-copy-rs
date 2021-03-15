#[cfg(test)]

use crate::paths::PathParser;

#[test]
fn test_append_path() {
    let path1 = "C:\\Users";
    let path2 = "jfast\\Desktop";
    let full_path = "C:\\Users\\jfast\\Desktop";
    let pp1 = PathParser::new(&String::from(path1));
    let pp2 = pp1.append_segment(&String::from(path2));
    let result = pp2.get_segment().get_default_segment_string();
    let comparator = PathParser::new(&String::from(full_path))
        .get_segment()
        .get_default_segment_string();
    assert_eq!(result, comparator);
}

#[test]
fn test_path_comparison() {
    let path1 = "C:\\Users\\jfast\\Desktop";
    let path2 = "C:\\Users\\jfast\\Desktop";
    let pp1 = PathParser::new(&String::from(path1));
    let pp2 = PathParser::new(&String::from(path2));
    let pp1_segment = pp1.get_segment();
    let pp2_segment = pp2.get_segment();
    let contains = pp1_segment.contains_all_of_segment(&pp2_segment);
    assert_eq!(contains, true);
}

