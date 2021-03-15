use structopt::StructOpt;

struct Generate {

}

#[derive(StructOpt, Debug)]
pub enum Command {
    Genkey
}

impl Command {
    fn generate_key(self) {
        println!("generating key");
    }

    pub fn run(self) {
        match self {
            Command::Genkey => self.generate_key(),
        }
    }
}
