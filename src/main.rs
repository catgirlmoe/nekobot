use serenity::{
    async_trait,
    client::{
        bridge::gateway::GatewayIntents,
        EventHandler,
        Context,
        Client,
    },
    model::{
        channel::{Message, Reaction, ReactionType},
        prelude::{User, Member, RoleId, GuildId},
        gateway::Ready,
        id::ChannelId
    },
    framework::standard::{
        macros::{command, group},
        StandardFramework,
        CommandResult,
    }
};

use std::env;


const COLOR_ROLES: [RoleId; 8] = [
    RoleId(816297710662582322), RoleId(816298008412028968),
    RoleId(816298168945475595), RoleId(816298943926763551),
    RoleId(816298721376731177), RoleId(812122334248435742),
    RoleId(812123073406042113), RoleId(816419487385518101)
];

const COLOR_REACTS: [&str; 8] = [
    "🍓", "🍊", "🍌", "🥝", "🫐", "🍇", "🍒", "🌸"
];

const OPT_ROLES: [RoleId; 4] = [
    RoleId(818954872074272866), RoleId(824789571266281572),
    RoleId(819194863299592212), RoleId(824789788061073418)
];

const OPT_REACTS: [&str; 4] = [
    "👨‍💻", "⛩️", "🐱", "🎮"
];

// Event handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}#{}", ready.user.name, ready.user.discriminator);
    }

    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, member: Member) {
        ChannelId(808467694100938772).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|ea| {
                    ea.icon_url(member.user.avatar_url().expect("fuck icons"));
                    ea.name(format!("{}#{}", member.user.name, member.user.discriminator));
                    ea.url("https://catgirl.moe/todo/profile/page");
                    ea
                });
                e.colour(Colour::from_rgb(139, 195, 74));
                e.description(    
                    format!("Welcome <@{}> to the server!\nEnjoy your stay!", member.user.id)
                );
                e
            });
            m
        }).await.unwrap();
    }

    async fn guild_member_removal(&self, ctx: Context, _guild_id: GuildId, user: User, _member: Option<Member>) {
        ChannelId(808467694100938772).send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.author(|ea| {
                    ea.icon_url(user.avatar_url().expect("fuck icons"));
                    ea.name(format!("{}#{}", user.name, user.discriminator));
                    ea.url(format!("https://catgirl.moe/member/{}", user.id));
                    ea
                });
                e.colour(Colour::from_rgb(244, 67, 54));
                e.description(    
                    format!("<@{}> has left the server!\nWe will miss you!", user.id)
                );
                e
            });
            m
        }).await.unwrap();
    }


    async fn reaction_add(&self, ctx: Context, r: Reaction) {
        if r.user_id.expect("What the fuck") == 810172289462435881 {return;}
        if r.message_id == 818504538305593385 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = COLOR_REACTS.iter().enumerate().filter(|&(_, reacts)| r.emoji.unicode_eq(reacts)).map(|(i, _)| COLOR_ROLES[i]).collect();
            for rea in reacts {
                ctx.cache.member(&guild, &user).await.expect("Failed to get ,e,ner to add role").add_role(&ctx.http, rea).await.expect("Failed to add role");
            }
            let mem_roles: Vec<_> = m.roles.iter().filter(|role| COLOR_ROLES.contains(role)).collect();
            for role in mem_roles {
                ctx.cache.member(&guild, &user).await.expect("Failed to get member to remove role").remove_role(&ctx.http, role).await.expect("Failed to remove role");
            }
            r.delete(&ctx.http).await.expect("Failed to delete reaction");
        }
        if r.message_id == 824811302605029397 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = OPT_REACTS.iter().enumerate().filter(|&(_, reacts)| r.emoji.unicode_eq(reacts)).map(|(i, _)| OPT_ROLES[i]).collect();
            for rea in reacts {
                ctx.cache.member(&guild, &user).await.expect("Failed to get ,e,ner to add role").add_role(&ctx.http, rea).await.expect("Failed to add role");
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, r: Reaction) {
        if r.user_id.expect("What the fuck") == 810172289462435881 {return;}
        if r.message_id == 824811302605029397 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = OPT_REACTS.iter().enumerate().filter(|&(_, reacts)| r.emoji.unicode_eq(reacts)).map(|(i, _)| OPT_ROLES[i]).collect();
            for rea in reacts {
                ctx.cache.member(&guild, &user).await.expect("Failed to get ,e,ner to add role").remove_role(&ctx.http, rea).await.expect("Failed to add role");
            }
        }
    }


}



#[group]
#[commands(color, optin)]
struct General;

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");
    

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

use serenity::utils::Colour;

#[command]
async fn optin(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 638230362711130132 {return Ok(())}
    let sent = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("Pick an opt-in role!");
            e.colour(Colour::from_rgb(233, 30, 99));
            e.description(
                "<@&818954872074272866>
                <@&824789571266281572>
                <@&819194863299592212>
                <@&824789788061073418>"
            );
    
            e
        });
    
        m
    }).await.unwrap();
    for reacts in &OPT_REACTS {
        sent.react(&ctx.http, ReactionType::Unicode(reacts.to_string())).await.expect("Fucking bruuuuh");
    }
    Ok(())
}

#[command]
async fn color(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 638230362711130132 {return Ok(())}
    let sent = msg.channel_id.send_message(&ctx.http, |m| {
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
                <@&816419487385518101>"
            );
    
            e
        });
    
        m
    }).await.unwrap();
    for reacts in &COLOR_REACTS {
        sent.react(&ctx.http, ReactionType::Unicode(reacts.to_string())).await.expect("Fucking bruuuuh");
    }
    Ok(())
}
