use std::path::{Path};
use git2::{Repository};
use regex::Regex;
use std::env;

mod group;

use self::group::Group;

#[derive(Debug)]
struct Commit<'a> {
    summary: String, // TODO: try to convert this so str
    emoji_code: String,
    group: &'a Group<'a>,
}

impl<'a> Commit<'a> {
    fn parse(summary: &str, groups: &'a [Group<'a>]) -> Option<Commit<'a>> {
        let re = Regex::new(r":(.*?):(.*)").unwrap(); // TODO: const ?

        match re.captures(summary) {
            None => None,
            Some(captures) => {
                let emoji_code = captures.get(1).unwrap().as_str();
                let summary = captures.get(2).unwrap().as_str().trim();
                let group = match groups.iter().find(|group| group.codes.iter().any(|&code| code == emoji_code)) {
                    None => groups.iter().find(|group| group.name == "Others").unwrap(),
                    Some(group) => group,
                };

                Some(Commit {
                    summary: summary.to_string(),
                    emoji_code: emoji_code.to_string(),
                    group,
                })
            }
        }
    }
}

fn main() {
    let repository_local_path = env::args().nth(1).unwrap();
    let repository_local_path = Path::new(&repository_local_path);
    let repository = Repository::open(repository_local_path).unwrap();
    let mut revwalk = repository.revwalk().unwrap();
    revwalk.push_head();

    let groups = Group::all();

    let commits = revwalk
        .map(|oid| repository.find_commit(oid.unwrap()).unwrap())
        .map(|raw_commit| Commit::parse(raw_commit.summary().unwrap(), groups.as_slice()))
        .collect::<Vec<_>>();

    println!("{:#?}", commits);

    println!("done");
}
