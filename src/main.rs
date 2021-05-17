use dirs;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value as Toml;
use std::env;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    fpath: Option<PathBuf>,
}

pub fn get_file_contents(fpath: &str) -> String {
    let mut f = File::open(fpath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    NotInstalled,
    Installed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stack {
    pub name: String,
    pub add: Option<String>,
    pub source: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub stacks: Vec<Stack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Procedure {
    pub script_add: String,
    pub script_source: String,
}

impl Procedure {
    pub fn save(self) {
        // let build_dir = dirs::config_dir()
        // let build_dir = dirs::home_dir()
        //     .expect("Impossible to get your home dir")
        //     .join(".config/tokimk/build");
        // let build_dir = "".to_string();
        // fs::create_dir_all(&build_dir).unwrap();

        let path = env::current_dir().expect("error");
        let mut file = File::create(path.join("script_add.sh")).unwrap();
        writeln!(&mut file, "{}", &self.script_add).unwrap();

        let mut file = File::create(path.join("script_source.sh")).unwrap();
        writeln!(&mut file, "{}", &self.script_source).unwrap();
    }
}

impl Config {
    fn load(fpath: &PathBuf) -> Self {
        let mut f = File::open(fpath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        /// toml validation
        match contents.parse::<Toml>() {
            Ok(toml_string) => {}
            Err(error) => panic!("failed to parse TOML: {}", error),
        }

        let this: Self = toml::from_str(&contents).expect("failed to parse TOML");
        this
    }

    pub fn to_procedure(self) -> Procedure {
        Procedure {
            script_add: self.clone().gen_scritp_add(),
            script_source: self.clone().gen_scritp_source(),
        }
    }

    fn gen_scritp_add(self) -> String {
        self.stacks
            .into_iter()
            .map(|stack| stack.add.unwrap_or("".to_owned()))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn gen_scritp_source(self) -> String {
        self.stacks
            .into_iter()
            .map(|stack| stack.source.unwrap_or("".to_owned()))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn save(self, fpath: &str) {
        let mut file = File::create(fpath).unwrap();

        let content = toml::to_string(&self).expect("Could not write to file");
        writeln!(&mut file, "{}", content).unwrap();
    }
}

fn main() {
    let opt = Opt::from_args();
    let fpath = match opt.fpath {
        Some(fpath) => fpath,
        None => dirs::home_dir()
            .expect("Impossible to get your home dir")
            .join(".tokimk.toml"),
    };
    let config = Config::load(&fpath);

    let p = config.to_procedure();
    p.save();
}
