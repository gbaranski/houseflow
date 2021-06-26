use crate::{Command, Target};
use anyhow::Context;
use async_trait::async_trait;
use clap::Clap;

#[derive(Clap)]
pub struct ConfigGenerateCommand {
    target: Option<Target>,

    #[clap(short = 'f', long = "force")]
    force: bool,
}

#[async_trait(?Send)]
impl Command<()> for ConfigGenerateCommand {
    async fn run(&self, _state: ()) -> anyhow::Result<()> {
        let create_config = |target: Target| async move {
            let config = match target {
                Target::Server => houseflow_config::server::Config::default_toml(),
                Target::Client => houseflow_config::client::Config::default_toml(),
                Target::Device => houseflow_config::device::Config::default_toml(),
            };
            let path = target.config_path();
            if path.exists() && !self.force {
                println!(
                    "{} config already exists, use -f argument to overwrite",
                    target
                );
                return Ok(());
            }
            tokio::fs::create_dir_all(path.parent().unwrap()).await?;

            use tokio::fs::File;
            use tokio::io::AsyncWriteExt;

            let mut file = File::create(&path).await?;
            file.write(config.as_bytes()).await?;
            println!(
                "✅ Generated {} config at {}",
                target,
                path.to_str().unwrap()
            );

            Ok(())
        };

        match self.target.clone() {
            Some(target) => create_config(target).await,
            None => {
                use futures::future::join_all;
                use strum::IntoEnumIterator;
                let iter = Target::iter()
                    .map(|target| async { (target.clone(), create_config(target).await) });
                let results = join_all(iter).await;
                for (target, result) in results {
                    result.with_context(|| format!("when generating {} config", target))?;
                }

                Ok(())
            }
        }
    }
}
