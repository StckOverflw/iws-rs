use std::{str::FromStr, sync::Arc};

use rand::{distributions::Alphanumeric, Rng};
use twilight_model::{channel::message::MessageFlags, id::Id};
use twilight_util::builder::InteractionResponseDataBuilder;
use zephyrus::{
    prelude::{command, DefaultCommandResult, SlashContext},
    twilight_exports::{InteractionResponse, InteractionResponseType},
};

use crate::{
    checks::owner_command,
    database::{IWSCollections, Invitation},
};

#[command]
#[description = "Erstellen eines Invite für einen Server (bot owner)"]
#[checks(owner_command)]
#[required_permissions(MANAGE_GUILD)]
pub async fn invite(
    ctx: &SlashContext<Arc<IWSCollections>>,
    #[description = "guild id"] guild_id: String,
) -> DefaultCommandResult {
    ctx.interaction_client
        .create_response(
            ctx.interaction.id,
            &ctx.interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: Some(
                    InteractionResponseDataBuilder::new()
                        .flags(MessageFlags::EPHEMERAL)
                        .build(),
                ),
            },
        )
        .await?;

    let url = create_invite(ctx.data, &guild_id).await?;

    ctx.interaction_client
        .update_response(&ctx.interaction.token)
        .content(Some(&format!("Invite: <{}>", url)))
        .unwrap()
        .await?;

    Ok(())
}

pub async fn create_invite(collections: &IWSCollections, guild_id: &str) -> eyre::Result<String> {
    let invite: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    let _ = collections
        .invites
        .insert_one(
            Invitation {
                invite: invite.clone(),
                guild_id: Id::from_str(guild_id).unwrap(),
            },
            None,
        )
        .await?;

    let url = url::Url::parse(&format!(
        "{}/invitation/accept/{}",
        std::env::var("WEBSERVER_URL").unwrap(),
        invite
    ))
    .unwrap()
    .to_string();

    Ok(url)
}
