use std::collections::HashMap;
#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub params: HashMap<String, String>,
    pub default_instance: String,
    pub theme: HashMap<String, String>,
}
//Default Configs
impl Default for Config {
    fn default() -> Self {
        let mut params = HashMap::new();
        params.insert(String::from("limit"), String::from("10"));
        params.insert(String::from("sort"), String::from("Active"));
        params.insert(String::from("type_"), String::from("All"));
        let mut theme = HashMap::new();
        theme.insert(String::from("primary"), String::from("LightGreen"));
        theme.insert(String::from("secondary"), String::from("White"));
        theme.insert(String::from("bg"), String::from("Black"));
        Self {
            params: params,
            default_instance: String::from("https://lemmy.ml"),
            theme: theme,
        }
    }
}
impl Config {
    pub fn stringify(self) -> String {
        let mut str: String = String::from("");
        for (key, value) in &self.params {
            str += &format!("&{}={}", key, value);
        }
        return str;
    }
}
