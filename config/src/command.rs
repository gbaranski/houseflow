use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Command(String);

impl Command {
    pub fn execute(&self) -> Result<Vec<u8>, std::io::Error> {
        self.command().output().map(|v| v.stdout)
    }

    pub fn command(&self) -> std::process::Command {
        let mut command = std::process::Command::new("bash");
        command.arg("-c");
        command.arg(&self.0);
        command
    }
}
