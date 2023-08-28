use clap::{arg, value_parser, Arg, Command};
use std::ffi::OsString;

pub struct UserInput {
    pub url: String,
    pub port: u16,
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
        .get_matches_from(args);
    let url = matches
        .get_one::<String>("url")
        .expect("required")
        .to_owned();
    let port = matches.get_one::<u16>("port").expect("required").to_owned();
    UserInput { url, port }
}
