use clap::{Arg, ArgAction, Command};
use serde::{Serialize, Deserialize};
use serde_json;
use std::{fs::File, io::{BufReader, Result}};

pub fn get_arg() -> Result<(Config, bool)>
{
    let args = Command::new("OJ")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .required(true)
            .default_value("./config.json"))
        .arg(Arg::new("flush")
            .short('f')
            .long("flush-data")
            .action(ArgAction::SetTrue))
        .get_matches();

    let config = get_config(args.get_one::<String>("config").unwrap())?;
    let flush: bool = args.get_flag("flush");

    Ok((config, flush))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config
{
    pub server: Server,
    pub problems: Vec<Problem>,
    pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server
{
    pub bind_address: String,
    pub bind_port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Problem
{
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: String,
    pub misc: serde_json::Value,
    pub cases: Vec<Case>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Case
{
    pub score: u32,
    pub input_file: String,
    pub answer_file: String,
    pub time_limit: u64,
    pub memory_limit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language
{
    pub name: String,
    pub file_name: String,
    pub command: Vec<String>,
}

fn get_config(file_name: &str) -> Result<Config>
{
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}