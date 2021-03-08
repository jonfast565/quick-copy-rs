pub fn string_match(needle: String, haystack: String) -> bool {
    let needle_lower = needle.to_lowercase();
    let haystack_lower = needle.to_lowercase();

    for i in needle_lower.matches(haystack_lower.as_str()) {
        return true
    }

    false
}

pub fn string_match_str(needle: &str, haystack: &str) -> bool {
    string_match(String::from(needle), String::from(haystack))
}

pub fn char_match(needle: char, haystack: char) -> bool {
    let needle_lower = needle.to_lowercase().collect::<Vec<char>>();
    let haystack_lower = needle.to_lowercase().collect::<Vec<char>>();

    haystack_lower[0] == needle_lower[0]
}

pub fn path_is_unc(path: String) -> bool {
    path.starts_with("\\\\")
}