use structopt::StructOpt;


#[derive(StructOpt, Debug)]
pub struct Add {
    #[structopt(long)]
    pub name: String,
}

impl Add {
    fn run(self) {
        println!("adding device, name: {}", self.name);
    }
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Add(Add),
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Add(cmd) => cmd.run(),
        }

    }
}
