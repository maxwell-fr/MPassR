//! The default word list for word-based Random Token Generators
//! A variation of this list is used when no token list is specified.

/// Get a list of easy-to-recognize ASCII symbols
pub fn get_ez_ascii_symbols() -> Vec<char> {
    vec!['!', '@', '#', '$', '%', '&', '*', '+', '-']
}

/// Get a list of words hopefully acceptable to users of all literacy levels.
///
/// This list consists of approximately 500 words. The list was generated by
/// taking a list of the most common English nouns, constraining length (3-6 letters),
/// and curating it to remove anything that could be considered sensitive, workplace
/// inappropriate, or difficult to spell.
///
/// The intent is to provide a list that, when used to generate a passphrase that will be provided
/// to a user whose language skills are minimal, will hopefully reduce spelling errors
/// and other communication problems.
///
pub fn get_simpleton_words() -> Vec<&'static str> {
    vec![
        "time", "way", "year", "work", "day", "world", "life", "part", "house", "course", "case",
        "place", "end", "group", "party", "school", "fact", "money", "point", "state", "night",
        "area", "water", "thing", "head", "hand", "order", "side", "home", "week", "power", "room",
        "market", "lot", "car", "road", "form", "face", "sort", "office", "person", "health",
        "name", "book", "level", "view", "door", "line", "south", "city", "center", "effect",
        "staff", "kind", "job", "action", "act", "north", "age", "idea", "west", "moment", "sense",
        "report", "mind", "change", "land", "care", "range", "table", "back", "trade", "study",
        "street", "rate", "word", "food", "result", "team", "other", "sir", "air", "role",
        "reason", "price", "town", "class", "nature", "union", "bank", "member", "value", "need",
        "east", "type", "paper", "date", "figure", "right", "club", "voice", "stage", "king",
        "light", "tax", "march", "art", "board", "may", "month", "music", "cost", "field", "award",
        "issue", "bed", "game", "amount", "basis", "series", "top", "news", "front", "future",
        "rest", "labor", "hair", "bill", "heart", "force", "letter", "model", "growth", "fire",
        "chance", "sea", "record", "size", "space", "term", "plan", "energy", "income", "cup",
        "design", "choice", "hall", "list", "loss", "county", "wall", "hotel", "sun", "summer",
        "set", "color", "floor", "season", "unit", "park", "hour", "test", "garden", "style",
        "look", "deal", "charge", "help", "new", "page", "risk", "advice", "event", "fish", "oil",
        "doctor", "film", "press", "window", "shop", "access", "region", "doubt", "degree",
        "sound", "site", "title", "return", "public", "glass", "answer", "earth", "leader",
        "river", "eye", "appeal", "task", "sale", "whole", "method", "source", "piece", "lack",
        "demand", "post", "mouth", "radio", "sector", "firm", "status", "peace", "show", "arm",
        "base", "safety", "box", "past", "weight", "start", "league", "budget", "share", "cash",
        "aid", "rule", "tea", "truth", "turn", "smith", "review", "minute", "duty", "stone", "dog",
        "speech", "queen", "stock", "effort", "career", "length", "horse", "plant", "visit",
        "ball", "memory", "bar", "impact", "scale", "image", "trust", "edge", "gas", "gold",
        "wood", "text", "forest", "sister", "chair", "cause", "foot", "rise", "half", "winter",
        "corner", "step", "damage", "credit", "speed", "crime", "hill", "debate", "supply", "wind",
        "band", "museum", "farm", "pound", "match", "animal", "skin", "scene", "stuff", "play",
        "island", "claim", "video", "move", "target", "trial", "rock", "spirit", "coffee", "phone",
        "train", "sight", "factor", "battle", "grant", "tree", "bridge", "shape", "wine", "star",
        "hope", "detail", "path", "client", "search", "rain", "offer", "goal", "dinner", "while",
        "seat", "manner", "favor", "pair", "smile", "prince", "danger", "call", "output", "note",
        "tour", "middle", "track", "card", "sign", "player", "threat", "notice", "bottom", "fund",
        "file", "profit", "route", "second", "cell", "castle", "lead", "option", "reform",
        "spring", "touch", "estate", "volume", "youth", "boat", "branch", "bus", "waste", "heat",
        "neck", "object", "driver", "code", "crown", "entry", "bag", "coal", "leg", "belief",
        "total", "major", "aim", "flight", "treaty", "copy", "acid", "lunch", "engine", "cross",
        "key", "vote", "judge", "sky", "breath", "row", "guide", "milk", "cover", "screen",
        "silver", "green", "run", "coast", "valley", "drive", "metal", "motion", "index", "sport",
        "tape", "flow", "iron", "trip", "lane", "pool", "hole", "flat", "pay", "noise", "author",
        "nation", "sample", "aspect", "agent", "ship", "van", "meal", "motor", "tone", "sheet",
        "beauty", "square", "vision", "spot", "brown", "crowd", "fuel", "desk", "sum", "fall",
        "diet", "soil", "reader", "shock", "fruit", "behalf", "deputy", "roof", "nose", "steel",
        "artist", "plate", "song", "grass", "ice", "talk", "link", "ring", "expert", "rail",
        "strike", "map", "lake", "border", "bottle", "bird", "cat", "guy", "dress", "theme",
        "error", "loan", "stress", "wealth", "walk", "focus", "chief", "sleep", "mass", "bath",
        "item", "decade", "beach", "sugar", "height", "writer", "panel", "dream", "port", "bread",
        "chain", "worker", "empire", "mirror", "travel", "block", "meat", "store", "self", "break",
        "drama", "skill", "round", "scope", "gate", "mill", "golf", "plane", "gap", "core", "fun",
        "snow", "limit"
    ]
}
