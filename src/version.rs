use std::cmp::Ordering;

use semver;
use git2::string_array::StringArray;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
pub struct Version {
  pub name: String, // TODO: &str?
  pub semver: Option<semver::Version>,
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
  pub fn from_tag_names(names: &StringArray) -> Vec<Version> {
    let re = Regex::new(r"v?(.*)").unwrap(); // TODO: const ?
    let mut tag_names = names.iter()
      .filter_map(|name| name)
      .map(|name| {
        let mut version = Version {
          name: name.to_string(),
          semver: None,
        };

        if let Some(captures) = re.captures(name) {
          if let Some(capture) = captures.get(1) {
            version.semver = semver::Version::parse(capture.as_str()).ok()
          }
        }

        version
      })
      .collect::<Vec<_>>();

    tag_names.sort();

    tag_names
  }
}
