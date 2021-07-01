mod generate;

use generate::ConfigGenerateCommand;

use crate::Command;
use async_trait::async_trait;
use clap::Clap;

#[derive(Clap)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub subcommand: ConfigSubcommand,
}

#[derive(Clap)]
pub enum ConfigSubcommand {
    /// Generates a new configuration for specified targets, if none specified then it will generate for all targets.
    Generate(ConfigGenerateCommand),
}

#[async_trait(?Send)]
impl Command<()> for ConfigCommand {
    async fn run(self, state: ()) -> anyhow::Result<()> {
        match self.subcommand {
            ConfigSubcommand::Generate(cmd) => cmd.run(state).await,
        }
    }
}
