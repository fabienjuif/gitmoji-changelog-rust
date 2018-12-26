#[derive(Debug)]
pub struct Group<'a> {
    pub name: &'a str,
    pub codes: Vec<&'a str>,
}

impl<'a> Group<'a> {
    pub fn all() -> Vec<Group<'a>>{
        let mut groups = Vec::new();

        groups.push(Group {
            name: "Added",
            codes: vec![
                "sparkles",
                "tada",
                "sparkles",
                "tada",
                "white_check_mark",
                "construction_worker",
                "chart_with_upwards_trend",
                "heavy_plus_sign",
                "loud_so jund",
            ],
        });

        groups.push(Group {
            name: "Added",
            codes: vec![
                "art",
                "zap",
                "lipstick",
                "rotating_light",
                "arrow_down",
                "arrow_up",
                "pushpin",
                "recycle",
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
        });

        groups.push(Group {
            name: "Changed",
            codes: vec![
                "art",
                "zap",
                "lipstick",
                "rotating_light",
                "arrow_down",
                "arrow_up",
                "pushpin",
                "recycle",
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
        });

        groups.push(Group {
            name: "Breaking changes",
            codes: vec![
                "boom",
            ],
        });

        groups.push(Group {
            name: "Deprecated",
            codes: vec![
            ],
        });

        groups.push(Group {
            name: "Removed",
            codes: vec![
                "fire",
                "heavy_minus_sign",
                "mute",
            ],
        });

        groups.push(Group {
            name: "Fixed",
            codes: vec![
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
        });

        groups.push(Group {
            name: "Security",
            codes: vec![
                "lock",
            ],
        });

        groups.push(Group {
            name: "Useless",
            codes: vec![
                "bookmark",
            ],
        });

        groups.push(Group {
            name: "Miscellaneous",
            codes: vec![
            ],
        });

        groups.push(Group {
            name: "Others",
            codes: vec![],
        });

        groups
    }
}
