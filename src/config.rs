use std::env;

use lazy_static::lazy_static;

lazy_static! {
    // Configure the client with your Discord bot token in the environment.
    pub static ref TOKEN: String = env::var("BOT_TOK")
        .expect("Bot token in the environment");
}

pub const BOT_INFO: &'static str = r#"Template Bot"#;