use std::cmp::Ordering;

use regex::Regex;
use semver;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::commit::Commit;
use crate::group::Group;

lazy_static! {
    static ref RE_REMOVE_V: Regex = Regex::new(r"v?(.*)").unwrap();
}

#[derive(Eq, PartialEq, Debug, Clone)]
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
            Some(captures) => match captures.get(1) {
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

    pub fn set_commits(&mut self, commits: Vec<Commit>) -> &mut Version {
        self.groups = Group::from_commits(commits);

        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Version {
        self.name = name.to_string();

        self
    }
}
