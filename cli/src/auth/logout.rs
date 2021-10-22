use crate::CommandContext;

pub struct Command {}

impl crate::Command for Command {
    fn run(self, ctx: CommandContext) -> anyhow::Result<()> {
        ctx.tokens.remove()?;
        tracing::info!("✔ Succesfully logged out");

        Ok(())
    }
}
