use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::{Context, Error};
use std::time::Instant;

/// Show this help menu
#[poise::command(track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Ping the bot
///
/// Check the bot's ping
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();

    let embed_color = Colour::DARK_GREY;

    let start_embed_msg = CreateEmbed::new().title("Pinging...").color(embed_color);

    let first_reply: CreateReply = CreateReply::default().embed(start_embed_msg);

    let msg = ctx.send(first_reply).await?;

    // record ping
    let elapsed_ms = start.elapsed().as_millis();

    // create the 2nd embed
    let result_embed_msg = CreateEmbed::new()
        .title("Pong!")
        .description(format!("{} ms", elapsed_ms))
        .color(embed_color);

    msg.edit(ctx, CreateReply::default().embed(result_embed_msg))
        .await?;

    Ok(())
}
