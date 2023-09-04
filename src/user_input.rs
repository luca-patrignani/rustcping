use chrono::Duration;
use clap::{arg, value_parser, Arg, Command};
use std::ffi::OsString;

pub struct UserInput {
    pub url: String,
    pub port: u16,
    pub timeout: Option<Duration>,
    pub probes_count: Option<u128>,
    pub interval_between_probes: Duration
}

impl Clone for UserInput {
    fn clone(&self) -> Self {
        Self { url: self.url.clone(), port: self.port, timeout: self.timeout, probes_count: self.probes_count, interval_between_probes: self.interval_between_probes }
    }
}

pub struct UserInputBuilder {
    user_input: UserInput
}

impl UserInputBuilder {
    pub fn new(url: String, port: u16) -> Self {
        UserInputBuilder { user_input: UserInput { url, port, 
            timeout: None, 
            probes_count: None, 
            interval_between_probes: Duration::seconds(1) 
        }}
    }

    pub fn probes_count(mut self, count: u128) -> Self {
        self.user_input.probes_count = Some(count);
        self
    }

    pub fn interval_between_probes(mut self, interval_between_probes: Duration) -> Self {
        self.user_input.interval_between_probes = interval_between_probes;
        self
    }

    pub fn build(self) -> UserInput {
        self.user_input
    }
}

pub fn parse<I, T>(args: I) -> UserInput
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = Command::new("test")
        .arg(Arg::new("url"))
        .arg(
            arg!(--port <VALUE>)
                .value_parser(value_parser!(u16))
                .default_value("443"),
        )
        .arg(
            arg!(--timeout <VALUE>)
                .value_parser(value_parser!(f32))
                .allow_hyphen_values(true)
                .default_value("1.0"),
        )
        .arg(
            arg!(--count <VALUE>)
                .value_parser(value_parser!(u128))
                .default_value("0")
        )
        .get_matches_from(args);
    let url = matches
        .get_one::<String>("url")
        .expect("required")
        .to_owned();
    let port = matches.get_one::<u16>("port").expect("required").to_owned();
    let timeout = matches
        .get_one::<f32>("timeout")
        .expect("required")
        .to_owned();
    if timeout < 0.0 {
        panic!("Timeout should be a positive number")
    }
    let count = matches.get_one::<u128>("count").expect("required").to_owned();
    UserInput {
        url,
        port,
        timeout: if timeout == 0.0 {None} else {Some(Duration::milliseconds((1000.0 * timeout) as i64))},
        probes_count: if count == 0 {None} else {Some(count)},
        interval_between_probes: Duration::seconds(1)
    }
}
