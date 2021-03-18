use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExecuteCommandDevice {
  /// Device ID, as per the ID provided in SYNC.
  pub id: String,
}

#[derive(Deserialize)]
pub struct ExecuteCommandExecution {
  /// The command to execute, usually with accompanying parameters.
  pub command: String,
  /// Aligned with the parameters for each command.
  pub params: std::collections::HashMap<String, String>
}

#[derive(Deserialize)]
pub struct ExecuteCommand {
  /// List of target devices.
  pub devices: Vec<ExecuteCommandDevice>,
  /// List of commands to execute on target devices.
  pub execution: ExecuteCommandExecution

}

#[derive(Deserialize)]
pub struct ExecutePayload {
  /// List of device target and command pairs.
  pub commands: Vec<ExecuteCommand>
}
