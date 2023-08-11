use std::io::Read;
use reqwest;
use local_ip_address::local_ip;
use whoami;
use regex::Regex;
use serde_json::json;


#[derive(Default, Debug)]
pub struct DiscordWebHook {
    pub url: String,
    pub username: String,
    pub avatar_url: String
}

impl DiscordWebHook {
    pub fn new(url: &str) -> Self {
        DiscordWebHook { url: url.to_owned(), username: format!("{}@{}", whoami::username(), whoami::hostname()), avatar_url: "https://l.ig4er.link/rqds-i".to_owned() }
    }

    pub fn new_with_username(url: &str, usermane: &str) -> Self {
        DiscordWebHook { url: url.to_owned(), username: usermane.to_owned(), avatar_url: "https://l.ig4er.link/rqds-i".to_owned() }
    }

    pub fn new_with_avatar(url: &str, usermane: &str, avatar_url: &str) -> Self {
        DiscordWebHook { url: url.to_owned(), username: usermane.to_owned(), avatar_url: avatar_url.to_owned() }
    }

    fn send(self, message: String) -> Result<u64, ()> {
        let client = reqwest::blocking::Client::new();
        if let Ok(res) = client.post(self.url)
        .header("Content-Type", "application/json")
        .body(json!({
            "username": self.username,
            "avatar_url": self.avatar_url,
            "content": message
        }).to_string())
        .send() {
            return Ok(res.content_length().unwrap());
        };
        Err(())
    }
}

#[derive(Default, Debug)]
pub struct Telemetry {
    pub hook: DiscordWebHook,
    pub app_name: String,
}

impl Telemetry {
    pub fn new(hook: DiscordWebHook, app_name: &str) -> Self {
        Telemetry{ hook: hook, app_name: app_name.to_owned() }
    }

    pub fn greet(self) -> Result<(), ()> {
        let message = self.get_telemetry_content();

        if let Ok(_) = self.hook.send(message) {
            return Ok(());
        };
        Err(())
    }

    pub fn msg(self, message: &str) -> Result<(), ()> {
        if let Ok(_) = self.hook.send(message.to_owned()) {
            Ok(())
        }
        else {
            Err(())
        }
    }

    fn get_telemetry_content(&self) -> String {
        format!("Hi, I'm currently running `{} {}`\nApplication: `{}` \nMy OS is `{} \"{}\"` under `{}`.\nMy device name is `{}`\nLanguages of my computer is `{}`\nMy local IP is `{}`\nMy external IP is `{}`", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), self.app_name, whoami::platform(), whoami::distro(), whoami::arch(), whoami::devicename(), whoami::lang().collect::<Vec<String>>().join(", "), local_ip().expect("Failed to get local IP"), Telemetry::get_external_ip())
    }

    fn get_external_ip() -> String {
        let ip_regex = Regex::new(r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}").unwrap();
        if let Ok(mut ip_resp) = reqwest::blocking::get("https://api.ipify.org?format=json") {
            let mut resp = String::new();
            let _ = ip_resp.read_to_string(&mut resp);
            if let Some(ip) = ip_regex.find(&resp) {
                return ip.as_str().to_owned();
            };
        };
        "Unknown".to_owned()
    }
}
