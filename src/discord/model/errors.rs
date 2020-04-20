use serenity::{
    model::prelude::Message,
    Result as SerenityResult
};

pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}