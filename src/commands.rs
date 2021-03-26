use serenity::client::Context;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::channel::{Message, ReactionType};

#[group]
#[commands(color, optin)]
pub struct General;

use serenity::utils::Colour;

#[command]
pub async fn optin(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 638230362711130132 {
        return Ok(());
    }
    let sent = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Pick an opt-in role!");
                e.colour(Colour::from_rgb(233, 30, 99));
                e.description(
                    "<@&818954872074272866>
                <@&824789571266281572>
                <@&819194863299592212>
                <@&824789788061073418>",
                );

                e
            });

            m
        })
        .await
        .unwrap();
    for reacts in &super::consts::OPT_REACTS {
        sent.react(&ctx.http, ReactionType::Unicode(reacts.to_string()))
            .await
            .expect("Fucking bruuuuh");
    }
    Ok(())
}

#[command]
pub async fn color(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 638230362711130132 {
        return Ok(());
    }
    let sent = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Pick a color role!");
                e.colour(Colour::from_rgb(233, 30, 99));
                e.description(
                    "<@&816297710662582322>
                <@&816298008412028968>
                <@&816298168945475595>
                <@&816298943926763551>
                <@&816298721376731177>
                <@&812122334248435742>
                <@&812123073406042113>
                <@&816419487385518101>",
                );

                e
            });

            m
        })
        .await
        .unwrap();
    for reacts in &super::consts::COLOR_REACTS {
        sent.react(&ctx.http, ReactionType::Unicode(reacts.to_string()))
            .await
            .expect("Fucking bruuuuh");
    }
    Ok(())
}
