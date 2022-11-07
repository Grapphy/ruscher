use lazy_static::lazy_static;
use regex::Regex;

pub fn process_content(content: &str) -> Vec<String> {
    lazy_static! {
        static ref REGEX_CMP: Regex =
            Regex::new("[a-zA-Z0-9._%+-]+@(?:[a-zA-Z0-9-]+.)+[a-zA-Z]{2,6}:(?:[a-zA-Z0-9]{5,})")
                .unwrap();
    }

    REGEX_CMP
        .find_iter(content)
        .into_iter()
        .filter_map(|val| val.as_str().parse().ok())
        .collect()
}
