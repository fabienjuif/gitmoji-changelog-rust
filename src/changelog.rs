use std::path::Path;

use git2::Repository;

use crate::commit::Commit;
use crate::version::Version;

#[derive(Debug, Serialize)]
pub struct Changelog {
    versions: Vec<Version>,
}

impl Changelog {
    pub fn from(path: &str, from: Option<&str>) -> Changelog {
        let repository = Path::new(&path);
        let repository = Repository::open(repository).unwrap();

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
        versions.iter_mut().for_each(|version| {
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

            version.set_commits(Commit::from_revwalk(&repository, &mut revwalk));

            previous_version_name = &version.name;
        });
        versions.reverse();

        Changelog { versions }
    }
}
