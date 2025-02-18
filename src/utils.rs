use serde_json::{json, Value};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    json!({
        "apiversion": "1",
        "author": "ni2scmn",
        "color": "#FFD700)",
        "head": "bee",
        "tail": "bolt",
    })
}
