#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;

use handlebars::Handlebars;

mod group;
mod commit;
mod changelog;
mod version;

use crate::changelog::Changelog;

const TEMPLATE: &str = r"
# Changelog
{{#each versions as |version|}}
## Version {{version.name}}
  {{#each version.commits as |commit|~}}
  - {{commit.emoji_code}} - {{commit.summary}}
  {{/each~}}
{{/each}}
";

fn main() {
    let repository = env::args().nth(1).unwrap();

    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..44b21e9d4b040ba4f36ce136c82a59659a68701b");
    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..HEAD");
    let changelog = Changelog::from_range(&repository, "v1.0.0..HEAD");

    let mut reg = Handlebars::new();
    println!(
        "{}",
        reg.render_template(TEMPLATE, &json!(changelog)).unwrap(),
    );

    // println!("{:#?}", changelog);
}
