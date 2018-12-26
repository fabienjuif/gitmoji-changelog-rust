use std::cmp::Ordering;

use semver;
use git2::Repository;
use regex::Regex;
use serde::ser::{Serialize, Serializer, SerializeStruct};

use crate::group::Group;
use crate::commit::Commit;

#[derive(Eq, PartialEq, Debug)]
pub struct Version {
  pub name: String, // TODO: &str?
  pub semver: Option<semver::Version>, // TODO: don't know if this is useful, except for sorting, I excluded it from JSON serialization
  pub groups: Vec<Group>,
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Version", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("groups", &self.groups)?;
        state.end()
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
      if self.semver.is_some() && other.semver.is_some() {
        return self.semver.cmp(&other.semver)
      }

      self.name.cmp(&other.name)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Version {
  pub fn new(name: &str) -> Version {
    Version {
      name: name.to_string(),
      semver: None,
      groups: vec![],
    }
  }

  pub fn from_repository(repository: &Repository) -> Vec<Version> {
    let re = Regex::new(r"v?(.*)").unwrap(); // TODO: const ?

    let mut versions = repository.tag_names(None).unwrap().iter()
      .filter_map(|name| name)
      .map(|name| {
        let mut version = Version::new(name);

        if let Some(captures) = re.captures(name) {
          if let Some(capture) = captures.get(1) {
            version.semver = semver::Version::parse(capture.as_str()).ok()
          }
        }

        version
      })
      .collect::<Vec<_>>();

    versions.sort();

    let mut revwalk = repository.revwalk().unwrap();
    let mut previous_version_name = String::from("");
    versions.iter_mut().for_each(|mut version| {
      revwalk.push_range(&format!("{}..{}", previous_version_name, version.name)).unwrap();

      version.groups = Group::from_commits(Commit::from_revwalk(&repository, &mut revwalk));

      previous_version_name = version.name.to_string();
    });
    versions.reverse();

    versions
  }
}
