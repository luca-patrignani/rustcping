use chrono::Duration;
use clap::{arg, value_parser, Arg, Command};
use std::ffi::OsString;

pub struct UserInput {
    pub url: String,
    pub port: u16,
    pub timeout: Option<Duration>,
    pub probes_count: Option<u128>,
    pub interval_between_probes: Duration,
}

impl Clone for UserInput {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            port: self.port,
            timeout: self.timeout,
            probes_count: self.probes_count,
            interval_between_probes: self.interval_between_probes,
        }
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
                .default_value("0"),
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
    let count = matches
        .get_one::<u128>("count")
        .expect("required")
        .to_owned();
    UserInput {
        url,
        port,
        timeout: if timeout == 0.0 {
            None
        } else {
            Some(Duration::milliseconds((1000.0 * timeout) as i64))
        },
        probes_count: if count == 0 { None } else { Some(count) },
        interval_between_probes: Duration::seconds(1),
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::user_input::parse;
    #[test]
    fn test_port() {
        assert_eq!(80, parse(["EXEC_NAME", "example.com", "--port", "80"]).port)
    }

    #[test]
    fn test_port_default() {
        assert_eq!(443, parse(["EXEC_NAME", "example.com"]).port)
    }

    #[test]
    fn test_url() {
        assert_eq!("example.com", parse(["EXEC_NAME", "example.com"]).url)
    }

    #[test]
    fn test_url_as_ip_addr() {
        assert_eq!("74.6.231.21", parse(["EXEC_NAME", "74.6.231.21"]).url)
    }

    #[test]
    fn test_timeout_as_int() {
        assert_eq!(
            Duration::seconds(3),
            parse(["EXEC_NAME", "1.2.3.4", "--timeout", "3"])
                .timeout
                .unwrap()
        )
    }

    #[test]
    fn test_timeout_as_float() {
        assert_eq!(
            Duration::milliseconds(3230),
            parse(["EXEC_NAME", "1.2.3.4", "--timeout", "3.23"])
                .timeout
                .unwrap()
        )
    }

    #[test]
    fn test_zero_timeout() {
        assert_eq!(
            None,
            parse(["EXEC_NAME", "1.2.3.4", "--timeout", "0"]).timeout
        )
    }

    #[test]
    #[should_panic]
    fn test_negative_timeout() {
        _ = parse(["EXEC_NAME", "1.2.3.4", "--timeout", "-2.3"])
    }

    #[test]
    fn test_probes_count_positive() {
        assert_eq!(
            Some(3),
            parse(["EXEC_NAME", "1.2.3.4", "--count", "3"]).probes_count
        )
    }

    #[test]
    fn test_probes_count_none() {
        assert_eq!(None, parse(["EXEC_NAME", "1.2.3.4"]).probes_count)
    }

    #[test]
    fn test_probes_count_none_zero() {
        assert_eq!(
            None,
            parse(["EXEC_NAME", "1.2.3.4", "--count", "0"]).probes_count
        )
    }
}
