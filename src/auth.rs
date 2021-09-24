use super::api;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use toml;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct AuthConfig {
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

pub fn login() -> Result<(String, String), Error> {
    let reader = stdin();
    let mut instance = String::new();
    let mut login = String::new();
    print!("Enter your instance: ");
    stdout().flush().unwrap();
    reader.read_line(&mut instance).ok().expect("");
    instance.pop();
    print!("Enter your username or email: ");
    stdout().flush().unwrap();
    reader.read_line(&mut login).ok().expect("");
    login.pop();
    print!("Enter your password: ");
    stdout().flush().unwrap();
    let pass = rpassword::read_password().unwrap_or_default();
    instance = super::utils::prepend_https(instance);
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
                        .open(&proj_dirs.config_dir().join("auth.toml"))?;
                    let config = fs::read_to_string(&proj_dirs.config_dir().join("auth.toml"))
                        .unwrap_or_default();
                    let mut toml: AuthConfig = toml::from_str(&config).unwrap_or_default();
                    toml.instancelist
                        .instances
                        .entry(instance.clone())
                        .or_insert(UserList::default())
                        .userlist
                        .insert(login, jwt.clone());
                    let new_config = toml::to_string(&toml).unwrap_or_default();
                    if let Ok(_) = write!(config_file, "{}", new_config) {
                        Ok((instance, jwt))
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
