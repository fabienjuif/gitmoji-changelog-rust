use std::path::Path;
use std::result::Result;

use git2::Repository;
use handlebars::Handlebars;

use crate::commit::Commit;
use crate::version::Version;

const TEMPLATE: &str = "{{#each versions as |version|}}
<a name=\"{{version.name}}\" data-comment=\"this line is used by gitmoji-changelog, don't remove it!\"></a>
## Version {{version.name}}
{{#each version.groups as |group|}}
### {{group.name}}
{{#each group.commits as |commit|~}}
 - {{commit.emoji}}  {{commit.summary}}{{#if @root.options.print-authors}} ({{commit.author}}){{/if}}
{{/each~}}
{{/each~}}
{{/each~}}
";

#[derive(Debug, Serialize)]
pub struct Changelog {
    pub versions: Vec<Version>,
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
                    if *from_version >= version {
                        return None;
                    }
                }

                Some(version)
            })
            .collect::<Vec<_>>();

        versions.sort();

        versions.push(Version::new("HEAD"));

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

    #[allow(dead_code)]
    pub fn keep_last_version_only(&self) -> Changelog {
        let mut versions = vec![];

        if let Some(last_version) = self.versions.first() {
            versions.push(last_version.clone());
        }

        Changelog { versions }
    }

    pub fn to_markdown(
        &self,
        release: Option<&str>,
        print_authors: bool,
    ) -> Result<String, String> {
        let mut versions = self.versions.clone();

        match release {
            None => {
                if let Some(version) = versions.first() {
                    if version.name == "HEAD" {
                        return Err(String::from("Your repository does not seem to contain any tag. Please use the release option if you wish to generate a changelog either way."));
                    }
                }

                if !versions.is_empty() {
                    versions.remove(0);
                }
            }
            Some(release_name) => {
                if let Some(recent_version) = versions.first_mut() {
                    if recent_version.name == "HEAD" {
                        recent_version.set_name(release_name);
                    }
                }
            }
        };

        let mut reg = Handlebars::new();
        reg.set_strict_mode(true);
        let json = json!({
            "versions": versions,
            "options": {
                "print-authors": print_authors,
            },
        });

        Ok(reg.render_template(TEMPLATE, &json).unwrap())
    }
}
