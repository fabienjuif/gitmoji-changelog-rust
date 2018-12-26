use std::collections::HashMap;

use crate::commit::Commit;

lazy_static! {
  static ref GROUPS: Vec<Group> = {
    let mut groups = vec![];

    groups.push(Group::new(
        "Added",
        vec![
            "sparkles",
            "tada",
            "sparkles",
            "tada",
            "white_check_mark",
            "construction_worker",
            "chart_with_upwards_trend",
            "heavy_plus_sign",
        ],
    ));

    groups.push(Group::new(
        "Changed",
        vec![
            "art",
            "zap",
            "lipstick",
            "rotating_light",
            "arrow_down",
            "arrow_up",
            "pushpin",
            "recycle",
            "hammer",
            "wrench",
            "rewind",
            "alien",
            "truck",
            "bento",
            "wheelchair",
            "speech_balloon",
            "card_file_box",
            "children_crossing",
            "building_construction",
            "iphone",
        ],
    ));

    groups.push(Group::new(
        "Breaking changes",
        vec![
            "boom",
        ],
    ));

    groups.push(Group::new(
        "Deprecated",
        vec![],
    ));

    groups.push(Group::new(
        "Removed",
        vec![
            "fire",
            "heavy_minus_sign",
            "mute",
        ],
    ));

    groups.push(Group::new(
        "Fixed",
        vec![
            "bug",
            "ambulance",
            "apple",
            "penguin",
            "checkered_flag",
            "robot",
            "green_apple",
            "green_heart",
            "pencil2",
        ],
    ));

    groups.push(Group::new(
        "Security",
        vec![
            "lock",
        ],
    ));

    groups.push(Group::new(
        "Useless",
        vec![
            "bookmark",
        ],
    ));

    groups.push(Group::new(
        "Miscellaneous",
        vec![],
    ));

    groups
  };
}

#[derive(Debug, Serialize, Eq)]
pub struct Group {
    pub name: &'static str,
    pub codes: Vec<&'static str>, // TODO: remove this from here
    pub commits: Vec<Commit>,
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.name == other.name
    }
}


impl Group {
    pub fn new(name: &'static str, codes: Vec<&'static str>) -> Group {
        Group{
            name,
            codes,
            commits: vec![],
        }
    }

    pub fn from_commits(commits: Vec<Commit>) -> Vec<Group> {
        let mut groups = HashMap::new();

        for commit in commits {
            // TODO: use a HashMap instead of doing this cardinal product
            let group_name = match GROUPS.iter().find(|group| group.codes.iter().any(|&code| code == commit.emoji_code)) {
                None => "Miscellaneous",
                Some(group) => group.name,
            };

            groups
                .entry(group_name)
                .or_insert(Group::new(group_name, vec![]));
            groups
                .entry(group_name)
                .and_modify(|group| group.commits.push(commit));
        }

        // Transform the HashMap to a Vec
        let mut vector = vec![];
        for (_, group) in groups {
            if group.name != "Useless" {
                vector.push(group);
            }
        }

        vector
    }
}
