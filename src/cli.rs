use clap::{arg, Args, Parser, Subcommand};
use crate::config::add_new_jira_project;


#[derive(Parser, Debug)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    RegisterJira(RegisterJira)
}

#[derive(Args, Debug)]
pub struct RegisterJira {
    #[arg(short, long)]
    pub link: String,
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
    #[arg(short, long)]
    pub company_name: String
}

impl RegisterJira {
    pub fn make_tuple_of_struct(&self) -> Vec<(String, &String)> {
        let args = vec![
            ("link".to_string(), &self.link),
            ("username".to_string(), &self.username),
            ("password".to_string(), &self.password),
            ("company_name".to_string(), &self.company_name),
        ];
        args
    }
}

pub fn parse_args() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::RegisterJira(reg) => {
            match add_new_jira_project(reg) {
                Ok(_) => (),
                Err(err) => println!("{:?}", err)
            };
            println!("CMD - {:?}", reg.link)
        }
    }
}
