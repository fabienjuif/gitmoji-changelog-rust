use std::cmp::Ordering;

use git2::Repository;
use regex::Regex;
use semver;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::commit::Commit;
use crate::group::Group;

lazy_static! {
  static ref RE_REMOVE_V: Regex = Regex::new(r"v?(.*)").unwrap();
}

#[derive(Eq, PartialEq, Debug)]
pub struct Version {
    pub name: String,                    // TODO: &str?
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
            return self.semver.cmp(&other.semver);
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
        let semver = match RE_REMOVE_V.captures(name) {
            None => None,
            Some(captures) =>  match captures.get(1) {
                None => None,
                Some(capture) => semver::Version::parse(capture.as_str()).ok(),
            },
        };

        Version {
            name: name.to_string(),
            semver,
            groups: vec![],
        }
    }

    pub fn from_repository(repository: &Repository, from: Option<&str>) -> Vec<Version> {
        let from_version = from.map(|f| Version::new(f));

        let mut versions = repository
            .tag_names(None)
            .unwrap()
            .iter()
            .filter_map(|name| name)
            .filter_map(|name| {
                let version = Version::new(name);

                if let Some(from_version) = &from_version {
                    if from_version >= &version {
                        return None;
                    }
                }

                Some(version)
            })
            .collect::<Vec<_>>();

        if versions.is_empty() {
            versions.push(Version::new("HEAD"));
        }

        versions.sort();

        let mut revwalk = repository.revwalk().unwrap();
        let mut previous_version_name = from.unwrap_or("");
        let versions_len = versions.len();
        versions.iter_mut().for_each(|mut version| {
            if version.name == "HEAD" && versions_len == 1 && from.is_none() {
                revwalk.push_head().unwrap();
            } else if previous_version_name == "" {
                revwalk
                    .push_ref(&format!("refs/tags/{}", version.name))
                    .unwrap();
            } else {
                revwalk
                    .push_range(&format!("{}..{}", previous_version_name, version.name))
                    .unwrap();
            }

            version.groups = Group::from_commits(Commit::from_revwalk(&repository, &mut revwalk));

            previous_version_name = &version.name;
        });
        versions.reverse();

        versions
    }
}
