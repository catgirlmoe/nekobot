use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::channel::Reaction;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::prelude::{GuildId, Member, User};
use serenity::utils::Colour;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "Connected as {}#{}",
            ready.user.name, ready.user.discriminator
        );
    }

    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, member: Member) {
        ChannelId(808467694100938772)
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|ea| {
                        ea.icon_url(member.user.avatar_url().expect("fuck icons"));
                        ea.name(format!(
                            "{}#{}",
                            member.user.name, member.user.discriminator
                        ));
                        ea.url(format!("https://catgirl.moe/u/{}", member.user.id));
                        ea
                    });
                    e.colour(Colour::from_rgb(139, 195, 74));
                    e.description(format!(
                        "Welcome <@{}> to the server!\nEnjoy your stay!",
                        member.user.id
                    ));
                    e
                });
                m
            })
            .await
            .unwrap();
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        user: User,
        _member: Option<Member>,
    ) {
        ChannelId(808467694100938772)
            .send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.author(|ea| {
                        ea.icon_url(user.avatar_url().expect("fuck icons"));
                        ea.name(format!("{}#{}", user.name, user.discriminator));
                        ea.url(format!("https://catgirl.moe/u/{}", user.id));
                        ea
                    });
                    e.colour(Colour::from_rgb(244, 67, 54));
                    e.description(format!(
                        "<@{}> has left the server!\nWe will miss you!",
                        user.id
                    ));
                    e
                });
                m
            })
            .await
            .unwrap();
    }
    async fn reaction_add(&self, ctx: Context, r: Reaction) {
        if r.user_id.expect("What the fuck") == 810172289462435881 {
            return;
        }
        if r.message_id == 818504538305593385 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = super::consts::COLOR_REACTS
                .iter()
                .enumerate()
                .filter(|&(_, reacts)| r.emoji.unicode_eq(reacts))
                .map(|(i, _)| super::consts::COLOR_ROLES[i])
                .collect();
            for rea in reacts {
                ctx.cache
                    .member(&guild, &user)
                    .await
                    .expect("Failed to get ,e,ner to add role")
                    .add_role(&ctx.http, rea)
                    .await
                    .expect("Failed to add role");
            }
            let mem_roles: Vec<_> = m
                .roles
                .iter()
                .filter(|role| super::consts::COLOR_ROLES.contains(role))
                .collect();
            for role in mem_roles {
                ctx.cache
                    .member(&guild, &user)
                    .await
                    .expect("Failed to get member to remove role")
                    .remove_role(&ctx.http, role)
                    .await
                    .expect("Failed to remove role");
            }
            r.delete(&ctx.http)
                .await
                .expect("Failed to delete reaction");
        }
        if r.message_id == 824811302605029397 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = super::consts::OPT_REACTS
                .iter()
                .enumerate()
                .filter(|&(_, reacts)| r.emoji.unicode_eq(reacts))
                .map(|(i, _)| super::consts::OPT_ROLES[i])
                .collect();
            for rea in reacts {
                ctx.cache
                    .member(&guild, &user)
                    .await
                    .expect("Failed to get ,e,ner to add role")
                    .add_role(&ctx.http, rea)
                    .await
                    .expect("Failed to add role");
            }
        }
    }
    async fn reaction_remove(&self, ctx: Context, r: Reaction) {
        if r.user_id.expect("What the fuck") == 810172289462435881 {
            return;
        }
        if r.message_id == 824811302605029397 {
            let guild = r.guild_id.unwrap();
            let user = r.user_id.unwrap();
            let m: Member = ctx.cache.member(&guild, &user).await.unwrap();
            let reacts: Vec<_> = super::consts::OPT_REACTS
                .iter()
                .enumerate()
                .filter(|&(_, reacts)| r.emoji.unicode_eq(reacts))
                .map(|(i, _)| super::consts::OPT_ROLES[i])
                .collect();
            for rea in reacts {
                ctx.cache
                    .member(&guild, &user)
                    .await
                    .expect("Failed to get ,e,ner to add role")
                    .remove_role(&ctx.http, rea)
                    .await
                    .expect("Failed to add role");
            }
        }
    }
}
