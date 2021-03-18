use structopt::StructOpt;

pub mod key;
pub mod device;
pub mod db;

#[derive(StructOpt, Debug)]
#[structopt(name = "houseflow", about = "CLI Application for houseflow")]
enum Command {
    Device(device::Command),
    Key(key::Command),
}

fn main() {
    let cmd: Command = Command::from_args();

    // Maybe simplify that later
    match cmd {
        Command::Device(cmd) => cmd.run(),
        Command::Key(cmd) => cmd.run(),
    }
}
