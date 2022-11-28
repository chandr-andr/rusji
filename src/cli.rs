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
    pub company_name: Option<String>
}

impl RegisterJira {
    pub fn make_tuple_of_struct(&self) -> Vec<(String, &String)> {
        let mut args = vec![
            ("link".to_string(), &self.link),
            ("username".to_string(), &self.username),
            ("password".to_string(), &self.password),
        ];
        match &self.company_name {
            Some(company_name) => {
                let company_name_tuple = ("company_name".to_string(), company_name);
                args.push(company_name_tuple);
                args
            },
            None => args
        }
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
