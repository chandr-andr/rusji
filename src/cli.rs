use clap::{arg, Args, Parser, Subcommand};


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
