use std::path::{Path};

use git2::Repository;

use crate::group::Group;
use crate::commit::Commit;
use crate::version::Version;

#[derive(Debug, Serialize)]
pub struct Changelog {
  range: String, // TODO: &str?
  versions: Vec<Version>,
}

impl Changelog {
  pub fn from_range(path: &str, range: &str) -> Changelog {
    let repository = Path::new(&path);
    let repository = Repository::open(repository).unwrap();

    Changelog {
      range: range.to_string().clone(),
      versions: Version::from_repository(&repository),
    }
  }
}
