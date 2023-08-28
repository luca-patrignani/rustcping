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

    pub struct ProbeBuilder {
        probe: Probe
    }
    
    impl ProbeBuilder {
        pub fn new() -> ProbeBuilder {
            ProbeBuilder { probe: Probe{
                start_time: Utc::now(),
                end_time: Utc::now(),
                err: None,
                cycle_duration: Duration::seconds(1)
            } }
        }
    
        pub fn start_time(mut self, time: DateTime<Utc>) -> ProbeBuilder {
            self.probe.start_time = time;
            self
        }

        pub fn end_time(mut self, time: DateTime<Utc>) -> ProbeBuilder {
            self.probe.end_time = time;
            self
        }
     
        pub fn err(mut self, err: std::io::Error) -> ProbeBuilder {
            self.probe.err = Some(err);
            self
        }
    
        pub fn cycle_duration(mut self, cycle_duration: Duration) -> ProbeBuilder {
            self.probe.cycle_duration = cycle_duration;
            self
        }
    
        pub fn build(self) -> Probe {
            self.probe
        }
    }

    fn dummy_error() -> Error {
        io::Error::new(io::ErrorKind::AddrInUse, "error")
    }

    fn success() -> Probe {
        ProbeBuilder::new().build()
    }

    fn failure() -> Probe {
        ProbeBuilder::new().err(dummy_error()).build()
    }

    #[test]
    fn test_counter() -> Result<(), AddrParseError> {
        let mut info = Info::new( 
            UserInput{ url: "example.com".to_owned(), port: 443 }, 
            IpAddr::from_str("93.184.216.34")? 
        );
        let success = &success();
        let failure = &failure();
        
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
            success()
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[0].start_time));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_single_fail() -> Result<(), AddrParseError> {
        let probes = [
            failure()
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[0].start_time));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_succ() -> Result<(), AddrParseError> {
        let probes = [
            success(),
            success(),
            success(),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[2].start_time));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_fail() -> Result<(), AddrParseError> {
        let probes = [
            failure(),
            failure(),
            failure(),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[2].start_time));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mixed() -> Result<(), AddrParseError> {
        let probes = [
            failure(),
            success(),
            failure(),
            success(),
            failure(),
        ];
        let info = create_info_from_probes(&probes)?;  
        assert_eq!(info.last_succ_probe, Some(probes[3].start_time));
        assert_eq!(info.last_fail_probe, Some(probes[4].start_time));
        Ok(())
    }

    #[test]
    fn test_total_uptime_downtime() -> Result<(), AddrParseError> {
        let probes = [
            ProbeBuilder::new().cycle_duration(Duration::seconds(2)).build(),
            ProbeBuilder::new().cycle_duration(Duration::seconds(3)).build(),
            ProbeBuilder::new().cycle_duration(Duration::seconds(2)).err(dummy_error()).build(),
            ProbeBuilder::new().cycle_duration(Duration::seconds(5)).build(),
            ProbeBuilder::new().cycle_duration(Duration::seconds(20)).err(dummy_error()).build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.total_uptime, Duration::seconds(10));
        assert_eq!(info.total_downtime, Duration::seconds(22));
        Ok(())
    }

    #[test]
    fn test_cons_longest_uptime() -> Result<(), AddrParseError> {
        let probes = [
            ProbeBuilder::new().start_time(Utc::now()).build(),
            ProbeBuilder::new().build(),
            ProbeBuilder::new().end_time(Utc::now()).build(),
            ProbeBuilder::new().err(dummy_error()).build()
        ];
        let info = create_info_from_probes(&probes)?;
        assert!(info.longest_cons_uptime.is_some());
        if let Some(longest_cons_uptime) = info.longest_cons_uptime {
            assert_eq!(longest_cons_uptime.0, probes[0].start_time);
            assert_eq!(longest_cons_uptime.1, probes[2].end_time);
        }
        Ok(())
    }

}
