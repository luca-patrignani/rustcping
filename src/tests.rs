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
    use std::{net::{AddrParseError, IpAddr}, str::FromStr, io::{self, Error}};

    use chrono::{Duration, Utc, DateTime};

    use crate::{tracker::{Info, Probe}, user_input::UserInput};

    fn dummy_error() -> Error {
        io::Error::new(io::ErrorKind::AddrInUse, "error")
    }

    fn success(time: DateTime<Utc>) -> Probe {
        Probe{ elapsed: Duration::seconds(1), err: None, time }
    }

    fn failure(time: DateTime<Utc>) -> Probe {
        Probe{ elapsed: Duration::seconds(1), err: Some(dummy_error()), time }
    }

    #[test]
    fn test_counter() -> Result<(), AddrParseError> {
        let mut info = Info::new( 
            UserInput{ url: "example.com".to_owned(), port: 443 }, 
            IpAddr::from_str("93.184.216.34")? 
        );
        let success = &success(Utc::now());
        let failure = &failure(Utc::now());
        
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 0);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(success);
        assert_eq!(info.succ_probes_streak, 1);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 1);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(success);
        assert_eq!(info.succ_probes_streak, 2);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 0);

        info.track(failure);
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 1);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 1);

        info.track(failure);
        assert_eq!(info.succ_probes_streak, 0);
        assert_eq!(info.fail_probes_streak, 2);
        assert_eq!(info.succ_probes_counter, 2);
        assert_eq!(info.fail_probes_counter, 2);

        info.track(success);
        assert_eq!(info.succ_probes_streak, 1);
        assert_eq!(info.fail_probes_streak, 0);
        assert_eq!(info.succ_probes_counter, 3);
        assert_eq!(info.fail_probes_counter, 2);

        Ok(())
    }

    fn create_info_from_probes(probes: &[Probe]) -> Result<Info, AddrParseError> {
        let mut info = Info::new( 
            UserInput{ url: "example.com".to_owned(), port: 443 }, 
            IpAddr::from_str("93.184.216.34")?
        );
        probes.iter().for_each(|probe| info.track(probe));
        Ok(info)
    }

    #[test]
    fn test_last_succ_and_fail_single_succ() -> Result<(), AddrParseError> {
        let probes = [
            success(Utc::now())
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[0].time));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_single_fail() -> Result<(), AddrParseError> {
        let probes = [
            failure(Utc::now())
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[0].time));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_succ() -> Result<(), AddrParseError> {
        let probes = [
            success(Utc::now()),
            success(Utc::now()),
            success(Utc::now()),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[2].time));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_fail() -> Result<(), AddrParseError> {
        let probes = [
            failure(Utc::now()),
            failure(Utc::now()),
            failure(Utc::now()),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[2].time));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mixed() -> Result<(), AddrParseError> {
        let probes = [
            failure(Utc::now()),
            success(Utc::now()),
            failure(Utc::now()),
            success(Utc::now()),
            failure(Utc::now()),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[3].time));
        assert_eq!(info.last_fail_probe, Some(probes[4].time));
        Ok(())
    }

}
