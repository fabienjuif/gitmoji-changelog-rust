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
{{#each changelog.versions as |version|}}
## Version {{version.name}}
{{#each version.groups as |group|}}
### {{group.name}}
{{#each group.commits as |commit|~}}
 - {{commit.emoji}}  {{commit.summary}}{{#if @root.options.print-authors}} ({{commit.author}}){{/if}}
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
            Arg::with_name("print-authors")
                .short("a")
                .long("print-authors")
                .help("Print author for each commit")
                .takes_value(false),
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
    let changelog = Changelog::open(&repository);

    let mut reg = Handlebars::new();
    reg.set_strict_mode(true);
    let json = json!({
        "changelog": changelog,
        "options": {
            "print-authors": matches.is_present("print-authors"),
        },
    });
    let result = reg.render_template(TEMPLATE, &json).unwrap();

    match matches.value_of("output") {
        None => println!("{}", result),
        Some(path) => {
            let mut file = File::create(path).unwrap();
            file.write_all(result.as_bytes()).unwrap();
        }
    }
}
