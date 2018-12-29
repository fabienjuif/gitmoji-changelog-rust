use git2::{Repository, Revwalk};
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_SUMMARY: Regex = Regex::new(r":(.*?):(.*)").unwrap();

    static ref EMOJIES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // You can use gen_code.js to generate this
        m.insert("hammer", "🔨");
        m.insert("art", "🎨");
        m.insert("zap", "⚡️");
        m.insert("fire", "🔥");
        m.insert("bug", "🐛");
        m.insert("ambulance", "🚑");
        m.insert("sparkles", "✨");
        m.insert("memo", "📝");
        m.insert("rocket", "🚀");
        m.insert("lipstick", "💄");
        m.insert("tada", "🎉");
        m.insert("white_check_mark", "✅");
        m.insert("lock", "🔒");
        m.insert("apple", "🍎");
        m.insert("penguin", "🐧");
        m.insert("checkered_flag", "🏁");
        m.insert("robot", "🤖");
        m.insert("green_apple", "🍏");
        m.insert("bookmark", "🔖");
        m.insert("rotating_light", "🚨");
        m.insert("construction", "🚧");
        m.insert("green_heart", "💚");
        m.insert("arrow_down", "⬇️");
        m.insert("arrow_up", "⬆️");
        m.insert("pushpin", "📌");
        m.insert("construction_worker", "👷");
        m.insert("chart_with_upwards_trend", "📈");
        m.insert("recycle", "♻️");
        m.insert("whale", "🐳");
        m.insert("heavy_plus_sign", "➕");
        m.insert("heavy_minus_sign", "➖");
        m.insert("wrench", "🔧");
        m.insert("globe_with_meridians", "🌐");
        m.insert("pencil2", "✏️");
        m.insert("hankey", "💩");
        m.insert("rewind", "⏪");
        m.insert("twisted_rightwards_arrows", "🔀");
        m.insert("package", "📦");
        m.insert("alien", "👽");
        m.insert("truck", "🚚");
        m.insert("page_facing_up", "📄");
        m.insert("boom", "💥");
        m.insert("bento", "🍱");
        m.insert("ok_hand", "👌");
        m.insert("wheelchair", "♿️");
        m.insert("bulb", "💡");
        m.insert("beers", "🍻");
        m.insert("speech_balloon", "💬");
        m.insert("card_file_box", "🗃");
        m.insert("loud_sound", "🔊");
        m.insert("mute", "🔇");
        m.insert("busts_in_silhouette", "👥");
        m.insert("children_crossing", "🚸");
        m.insert("building_construction", "🏗");
        m.insert("iphone", "📱");
        m.insert("clown_face", "🤡");
        m.insert("egg", "🥚");
        m.insert("see_no_evil", "🙈");
        m.insert("camera_flash", "📸");
        m.insert("alembic", "⚗");
        m.insert("mag", "🔍");
        m.insert("wheel_of_dharma", "☸️");
        m.insert("label", "🏷️");
        m
    };
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub struct Commit {
    pub hash: String,
    pub summary: String, // TODO: try to convert this so str
    pub author: String,
    pub emoji_code: String,
    pub emoji: String,
}

impl Commit {
    pub fn new(hash: String, summary: &str, author: &str, emoji_code: &str) -> Commit {
        Commit {
            hash: hash,
            summary: summary.to_string(),
            author: author.to_string(),
            emoji_code: emoji_code.to_string(),
            emoji: EMOJIES.get(emoji_code).unwrap_or(&emoji_code).to_string(),
        }
    }

    pub fn from_git2(commit: &git2::Commit) -> Option<Commit> {
        // author
        let author = commit.author();
        let author = author.name().or(author.email()).unwrap_or("");

        // hash (full, since git2r doesn't support shorthash yet )
        let hash = format!("{}", commit.id());

        // summary and new commit
        let summary = commit.summary().unwrap_or("");
        match RE_SUMMARY.captures(summary) {
            None => None,
            Some(captures) => {
                let emoji_code = captures.get(1).unwrap().as_str();
                let summary = captures.get(2).unwrap().as_str().trim();

                Some(Commit::new(hash, summary, author, emoji_code))
            }
        }
    }

    pub fn from_revwalk(repository: &Repository, revwalk: &mut Revwalk) -> Vec<Commit> {
        revwalk
            .filter_map(|oid| repository.find_commit(oid.unwrap()).ok())
            .filter_map(|raw_commit| Commit::from_git2(&raw_commit))
            .collect()
    }
}
