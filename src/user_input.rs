use chrono::Duration;
use clap::{arg, value_parser, Arg, Command};
use std::ffi::OsString;

pub struct UserInput {
    pub url: String,
    pub port: u16,
    pub timeout: Option<Duration>,
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
    let lim_timeout = if timeout == 0.0 {
        None
    } else {
        Some(Duration::milliseconds((1000.0 * timeout) as i64))
    };
    UserInput {
        url,
        port,
        timeout: lim_timeout,
    }
}
