use regex::Regex;

use crate::group::Group;

#[derive(Debug, Serialize)]
pub struct Commit {
    pub summary: String, // TODO: try to convert this so str
    pub emoji_code: String,
    pub group_code: String,
}

impl Commit {
    pub fn parse(summary: &str, groups: &[Group]) -> Option<Commit> {
        let re = Regex::new(r":(.*?):(.*)").unwrap(); // TODO: const ?

        match re.captures(summary) {
            None => None,
            Some(captures) => {
                let emoji_code = captures.get(1).unwrap().as_str();
                let summary = captures.get(2).unwrap().as_str().trim();
                let group_code = match groups.iter().find(|group| group.codes.iter().any(|&code| code == emoji_code)) {
                    None => String::from("Others"),
                    Some(group) => group.name.to_string(),
                };

                Some(Commit {
                    summary: summary.to_string(),
                    emoji_code: emoji_code.to_string(),
                    group_code,
                })
            }
        }
    }
}
