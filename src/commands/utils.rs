use poise::CreateReply;

use crate::{Context, Error};
use std::time::Instant;

/// Shows the bot's latency
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start_time = Instant::now();
    let msg = ctx.say("Calculating ping...").await?;
    let end_time = Instant::now();

    msg.edit(
        ctx,
        CreateReply::default().content(format!("{} ms", (end_time - start_time).as_millis())),
    )
    .await?;
    Ok(())
}

/// Hello world command
#[poise::command(prefix_command, slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}
