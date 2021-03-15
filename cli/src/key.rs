use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Generate {

}

impl Generate {
    pub fn run(self) {
        println!("generating key");
    }
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Generate(Generate)
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Generate(cmd) => cmd.run(),
        }
    }
}
