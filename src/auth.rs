use super::api;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{stdin, Error, ErrorKind, Write};
use toml;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Config {
    pub instancelist: InstanceList,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct InstanceList {
    pub instances: HashMap<String, UserList>,
}
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct UserList {
    pub userlist: HashMap<String, String>,
}

pub fn login() -> Result<(), Error> {
    let reader = stdin();
    let mut instance = String::new();
    let mut login = String::new();
    let mut pass = String::new();
    println!("Enter your instance");
    reader.read_line(&mut instance).ok().expect("");
    instance.pop();
    println!("Enter your username or email");
    reader.read_line(&mut login).ok().expect("");
    login.pop();
    println!("Enter your password");
    reader.read_line(&mut pass).ok().expect("");
    pass.pop();
    if !(instance.starts_with("https://")) {
        instance.insert_str(0, "https://")
    }
    match api::login(
        format!("{}/api/v3/user/login", instance),
        login.clone(),
        pass,
    ) {
        Ok(jwt) => {
            if !jwt.is_empty() {
                println!("Login successful");
                if let Some(proj_dirs) = ProjectDirs::from("dev", "ltv", "ltv") {
                    fs::create_dir_all(proj_dirs.config_dir())?;
                    let mut config_file = fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&proj_dirs.config_dir().join("ltv.toml"))?;
                    let config = fs::read_to_string(&proj_dirs.config_dir().join("ltv.toml"))
                        .unwrap_or_default();
                    let mut toml: Config = toml::from_str(&config).unwrap_or_default();
                    toml.instancelist
                        .instances
                        .entry(instance)
                        .or_insert(UserList::default())
                        .userlist
                        .insert(login, jwt);
                    let new_config = toml::to_string(&toml).unwrap_or_default();
                    if let Ok(_) = write!(config_file, "{}", new_config) {
                        Ok(())
                    } else {
                        Err(Error::new(ErrorKind::Other, "Couldn't save login details"))
                    }
                } else {
                    Err(Error::new(ErrorKind::Other, "Couldn't save login details"))
                }
            } else {
                Err(Error::new(ErrorKind::Other, "Login Failed"))
            }
        }
        Err(e) => {
            println!("Something went wrong {}", e);
            Err(Error::new(ErrorKind::Other, e))
        }
    }
}
