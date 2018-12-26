use std::env;

mod group;
mod commit;
mod changelog;

use crate::changelog::Changelog;

fn main() {
    let repository = env::args().nth(1).unwrap();

    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..44b21e9d4b040ba4f36ce136c82a59659a68701b");
    // let changelog = Changelog::from_range(&repository, "06a218d4bba6d3d7bf359bd9eff4013f585fc1fa..HEAD");
    let changelog = Changelog::from_range(&repository, "v1.0.0..HEAD");

    println!("{:#?}", changelog);
}
