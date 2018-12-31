use clap::{App, Arg, ArgMatches};

pub fn parse_args() -> ArgMatches<'static> {
    App::new("gitmoji-changelog")
    .version(crate_version!())
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
            .value_name("GIT_REPOSITORY_PATH")
            .help("Path to the git repository to parse")
            .required(true),
    )
    .arg(
        Arg::with_name("release")
            .short("r")
            .long("release")
            .help("Set a version to the release (latest tag to HEAD). If not set, the commits after the latest tag will not be printed to the changelog.")
            .takes_value(true)
            .required(false)
    )
    .arg(
        Arg::with_name("delta")
            .long("delta")
            .help("Print delta only (not the whole CHANGELOG).")
            .required(false)

    )
    .get_matches()
}
