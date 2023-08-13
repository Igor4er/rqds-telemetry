pub mod blocking;


#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;
    
    #[test]
    fn blocking_send_telemetry_greet() {
        dotenv().expect(".env file not found");
        let hook = blocking::DiscordWebHook::new(&env::var("TEST_DS_HOOK").unwrap());
        let telemetry = blocking::Telemetry::new(hook, "rqds-telemetry (testing)");
        let res = telemetry.greet();
        assert_eq!(res, Ok(()))
    }

    #[test]
    fn blocking_send_telemetry_msg() {
        dotenv().expect(".env file not found");
        let hook = blocking::DiscordWebHook::new(&env::var("TEST_DS_HOOK").unwrap());
        let telemetry = blocking::Telemetry::new(hook, "rqds-telemetry (testing)");
        let res = telemetry.msg("This is a `rqds-telemetry` test message. All should be ok if you see it in discord :thumbsup:");
        assert_eq!(res, Ok(()))
    }
}
