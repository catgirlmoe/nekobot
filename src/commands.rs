use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message};

use serde_json::json;

#[group]
#[commands(dothing)]
pub struct General;

#[command]
pub async fn dothing(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 638230362711130132 {
        return Ok(());
    }
    let ban = json!({
        "name": "status",
        "description": "Shows bot's host status",
        "options": [
            {
                "type": 5,
                "name": "extended",
                "description": "Display additional information thats excluded normally",
                "required": false
            }
        ]
    });
    println!("im gonna do the thing");
    //ctx.http.create_global_application_command(825877000022523994, &ban).await.expect("fuck something happened");
    Ok(())
}
