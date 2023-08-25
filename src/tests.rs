#[cfg(test)]
mod parse {
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
}

#[cfg(test)]
mod info {
    use std::{net::{AddrParseError, IpAddr}, str::FromStr, time::Duration, io::{self, Error}};

    use crate::{tracker::{Info, Probe}, user_input::UserInput};

    fn dummy_error() -> Error {
        io::Error::new(io::ErrorKind::AddrInUse, "error")
    }

    #[test]
    fn test_counter() -> Result<(), AddrParseError> {
        let mut info = Info::new( 
            UserInput{ url: "example.com".to_owned(), port: 443 }, 
            IpAddr::from_str("93.184.216.34")? 
        );
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 0);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(&Probe{ elapsed: Duration::from_secs(1), err: None });
        assert_eq!(info.succ_probes_streak, 1);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 1);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(&Probe{ elapsed: Duration::from_secs(1), err: None });
        assert_eq!(info.succ_probes_streak, 2);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(&Probe{ elapsed: Duration::from_secs(1), err: Some(dummy_error()) });
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 1);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 1);

        info.track(&Probe{ elapsed: Duration::from_secs(1), err: Some(dummy_error()) });
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 2);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 2);

        info.track(&Probe{ elapsed: Duration::from_secs(1), err: None });
        assert_eq!(info.succ_probes_streak, 1);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 3);
        assert_eq!(info.fail_probes_counter, 2);

        Ok(())
    }
}
