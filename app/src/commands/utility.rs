use crate::{Context, Error};

/// コマンドの登録を行います。
#[poise::command(slash_command, prefix_command, category = "Utility")]
pub(crate) async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    ctx.say(format!("Registering commands... Global is {}", global))
        .await?;
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

/// ヘルプを表示します。
#[poise::command(slash_command, prefix_command, track_edits, category = "Utility")]
pub(crate) async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to get help for"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "Extra text on bottom!",
        include_description: true,
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
