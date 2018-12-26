use regex::Regex;
use git2::{Repository, Revwalk};

use crate::group::Group;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Commit {
  pub summary: String, // TODO: try to convert this so str
  pub emoji_code: String,
  pub group_code: String,
}

impl Commit {
  pub fn new(summary: &str, emoji_code: &str, group_code: &str) -> Commit {
    Commit {
      summary: summary.to_string(),
      emoji_code: emoji_code.to_string(),
      group_code: group_code.to_string(),
    }
  }

  pub fn parse(summary: &str, groups: &[Group]) -> Option<Commit> {
      let re = Regex::new(r":(.*?):(.*)").unwrap(); // TODO: const ?

      match re.captures(summary) {
          None => None,
          Some(captures) => {
              let emoji_code = captures.get(1).unwrap().as_str();
              let summary = captures.get(2).unwrap().as_str().trim();
              // TODO: use a HashMap instead of doing this cardinal product
              let group_code = match groups.iter().find(|group| group.codes.iter().any(|&code| code == emoji_code)) {
                  None => "Others",
                  Some(group) => group.name,
              };

              Some(Commit::new(summary, emoji_code, group_code))
          }
      }
  }

  pub fn from_revwalk(repository: &Repository, revwalk: &mut Revwalk) -> Vec<Commit> {
    let groups = Group::all();

    revwalk
      .filter_map(|oid| repository.find_commit(oid.unwrap()).ok())
      .filter_map(|raw_commit| raw_commit.summary().map(|raw| raw.to_string()))
      .filter_map(|summary| Commit::parse(&summary, &groups))
      .collect()
  }
}
