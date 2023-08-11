pub mod blocking;


#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;
    
    #[test]
    fn send_telemetry_blocking() {
        dotenv().expect(".env file not found");
        let hook = blocking::DiscordWebHook{url: env::var("TEST_DS_HOOK").unwrap()};
        let telemetry = blocking::Telemetry::new(hook, "rqds-telemetry (testing)", None);
        let res = telemetry.greet();
        println!("{:?}", res);
    }
}
