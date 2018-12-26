use std::path::{Path};

use regex::Regex;
use git2::Repository;

use crate::commit::Commit;
use crate::group::Group;
use crate::version::Version;

#[derive(Debug)]
pub struct Changelog {
  range: String, // TODO: &str?
  commits: Vec<Commit>,
}

impl Changelog {
  pub fn from_range(path: &str, range: &str) -> Changelog {
    let repository = Path::new(&path);
    let repository = Repository::open(repository).unwrap();
    let mut revwalk = repository.revwalk().unwrap();

    let versions = Version::from_tag_names(&repository.tag_names(None).unwrap());
    println!("versions: {:?}", versions.iter().find(|version| version.name == "v1.0.0-alpha.0").map(|version| version.clone()));

    // TODO: range
    // revwalk.push_head();
    revwalk.push_range(range).unwrap();

    let groups = Group::all();

    Changelog {
      range: range.to_string().clone(),
      commits: revwalk
        .filter_map(|oid| repository.find_commit(oid.unwrap()).ok())
        .filter_map(|raw_commit| raw_commit.summary().map(|raw| raw.to_string()))
        .filter_map(|summary| Commit::parse(&summary, &groups))
        .collect(),
    }
  }
}
