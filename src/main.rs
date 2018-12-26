#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::prelude::*;

use std::env;

use clap::{App, Arg};
use handlebars::Handlebars;

mod changelog;
mod commit;
mod group;
mod version;

use crate::changelog::Changelog;

const TEMPLATE: &str = r"
# Changelog

{{#each versions as |version|}}
## Version {{version.name}}
{{#each version.groups as |group|}}
### {{group.name}}
{{#each group.commits as |commit|~}}
 - {{commit.emoji}}  {{commit.summary}}
{{/each~}}
{{/each~}}
{{/each~}}
";

fn main() {
    let matches = App::new("gitmoji-changelog")
        .version("1.0.0")
        .author("Fabien JUIF <fabien.juif@gmail.com>")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("File to update, if not defined write on stdout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .help("path to the git repository to parse")
                .value_name("GIT_REPOSITORY_PATH")
                .required(true),
        )
        .get_matches();

    eprintln!("Git repository path: {}", matches.value_of("path").unwrap());

    let repository = env::args().nth(1).unwrap();

    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..44b21e9d4b040ba4f36ce136c82a59659a68701b");
    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..HEAD");
    let changelog = Changelog::from_range(&repository, "v1.0.0..HEAD");

    let reg = Handlebars::new();
    let result = reg.render_template(TEMPLATE, &json!(changelog)).unwrap();

    match matches.value_of("output") {
        None => println!("{}", result),
        Some(path) => {
            let mut file = File::create(path).unwrap();
            file.write_all(result.as_bytes()).unwrap();
        }
    }
}
