use std::{net::SocketAddr, ffi::OsString};
use clap::{Command, arg, value_parser};

pub struct UserInput {
    pub socket: SocketAddr,
}

pub fn parse<I, T>(args: I) -> UserInput 
where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
{
    let matches = Command::new("test")
        .arg(arg!(--port <VALUE>).value_parser(value_parser!(u16)).default_value("443"))
        .get_matches_from(args);
    let port = matches.get_one::<u16>("port").expect("required").to_owned();
    UserInput{
        socket: SocketAddr::from(([93, 184, 216, 34], port))
    }
}
