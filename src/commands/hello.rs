use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hello there!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("hello")
        .description("Makes the bot say hi!")
}