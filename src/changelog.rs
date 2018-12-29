use std::path::Path;

use git2::Repository;

use crate::version::Version;

#[derive(Debug, Serialize)]
pub struct Changelog {
    versions: Vec<Version>,
}

impl Changelog {
    pub fn open(path: &str, from: Option<&str>) -> Changelog {
        let repository = Path::new(&path);
        let repository = Repository::open(repository).unwrap();

        Changelog {
            versions: Version::from_repository(&repository, from),
        }
    }
}
