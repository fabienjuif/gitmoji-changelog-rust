use std::cmp::Ordering;
use std::collections::HashMap;

use crate::commit::Commit;

lazy_static! {
    static ref GROUPS: Vec<Group> = {
        let mut groups = vec![];

        groups.push(Group::new(
            "Added",
            10,
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
            20,
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

        groups.push(Group::new("Breaking changes", 30, vec!["boom"]));

        groups.push(Group::new("Deprecated", 40, vec![]));

        groups.push(Group::new(
            "Removed",
            50,
            vec!["fire", "heavy_minus_sign", "mute"],
        ));

        groups.push(Group::new(
            "Fixed",
            60,
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

        groups.push(Group::new("Security", 70, vec!["lock"]));

        groups.push(Group::new("Miscellaneous", 80, vec![]));

        groups.push(Group::new("Useless", 90, vec!["bookmark"]));

        groups
    };
}

#[derive(Debug, Serialize, Eq, Clone)]
pub struct Group {
    pub order: usize,
    pub name: &'static str,
    pub codes: Vec<&'static str>, // TODO: remove this from here
    pub commits: Vec<Commit>,
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.name == other.name
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Group) -> Ordering {
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Group {
    pub fn new(name: &'static str, order: usize, codes: Vec<&'static str>) -> Group {
        Group {
            order,
            name,
            codes,
            commits: vec![],
        }
    }

    pub fn from_commits(commits: Vec<Commit>) -> Vec<Group> {
        let mut groups = HashMap::<&'static str, Group>::new();

        for commit in commits {
            // TODO: use a HashMap instead of doing this cardinal product
            let group = match GROUPS
                .iter()
                .find(|group| group.codes.iter().any(|&code| code == commit.emoji_code))
            {
                None => Group::new("Miscellaneous", 80, vec![]),
                Some(group) => group.clone(),
            };

            groups
                .entry(group.name)
                .or_insert(Group::new(group.name, group.order, vec![]));
            groups
                .entry(group.name)
                .and_modify(|group| group.commits.push(commit));
        }

        // Transform the HashMap to a Vec
        let mut vector = vec![];
        for (_, group) in groups {
            if group.name != "Useless" {
                vector.push(group);
            }
        }

        vector.sort();

        vector
    }
}
