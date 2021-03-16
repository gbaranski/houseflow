use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Generate {}

impl Generate {
    pub fn run(self) {
        let (pkey, skey) = sodiumoxide::crypto::sign::gen_keypair();
        println!("Public key: {}", base64::encode(pkey));
        println!("Private key: {}", base64::encode(skey));
    }
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Generate(Generate)
}

impl Command {
    pub fn run(self) {
        sodiumoxide::init().unwrap();
        match self {
            Command::Generate(cmd) => cmd.run(),
        }
    }
}
