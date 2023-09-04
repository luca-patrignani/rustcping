use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};

use crate::user_input::UserInput;

pub struct Probe {
    pub start: DateTime<Utc>,
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
    pub cycle_duration: Duration,
}

pub struct Info {
    pub user_input: UserInput,
    pub succ_probes_streak: u128,
    pub fail_probes_streak: u128,
    pub succ_probes_counter: u128,
    pub fail_probes_counter: u128,
    pub ip_addr: IpAddr,
    pub last_succ_probe: Option<DateTime<Utc>>,
    pub last_fail_probe: Option<DateTime<Utc>>,
    pub total_uptime: Duration,
    pub total_downtime: Duration,
    pub min_rtt: Duration,
    pub max_rtt: Duration,
    pub sum_rtt: Duration,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Info {
    pub fn new(user_input: UserInput, ip_addr: IpAddr) -> Info {
        Info {
            user_input,
            ip_addr,
            succ_probes_streak: 0,
            fail_probes_streak: 0,
            succ_probes_counter: 0,
            fail_probes_counter: 0,
            last_succ_probe: None,
            last_fail_probe: None,
            total_uptime: Duration::zero(),
            total_downtime: Duration::zero(),
            min_rtt: Duration::max_value(),
            max_rtt: Duration::min_value(),
            sum_rtt: Duration::zero(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn track(&mut self, probe: &Probe) {
        if probe.err.is_none() {
            self.succ_probes_streak += 1;
            self.fail_probes_streak = 0;
            self.succ_probes_counter += 1;
            self.last_succ_probe = Some(probe.start);
            self.total_uptime = self.total_uptime + probe.cycle_duration;
            self.min_rtt = Duration::min(self.min_rtt, probe.elapsed);
            self.max_rtt = Duration::max(self.max_rtt, probe.elapsed);
            self.sum_rtt = self.sum_rtt + probe.elapsed;
        } else {
            self.succ_probes_streak = 0;
            self.fail_probes_streak += 1;
            self.fail_probes_counter += 1;
            self.last_fail_probe = Some(probe.start);
            self.total_downtime = self.total_downtime + probe.cycle_duration;
        }
        if self.start_time.is_none() {
            self.start_time = Some(probe.start)
        }
        self.end_time = Some(probe.start + probe.elapsed);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Error},
        net::{AddrParseError, IpAddr},
        str::FromStr,
    };

    use chrono::{DateTime, Duration, Utc};

    use crate::{
        tracker::{Info, Probe},
        user_input::UserInput,
    };

    pub struct ProbeBuilder {
        probe: Probe,
    }

    impl ProbeBuilder {
        pub fn new() -> ProbeBuilder {
            ProbeBuilder {
                probe: Probe {
                    start: Utc::now(),
                    elapsed: Duration::seconds(1),
                    err: None,
                    cycle_duration: Duration::seconds(1),
                },
            }
        }

        pub fn start(mut self, time: DateTime<Utc>) -> ProbeBuilder {
            self.probe.start = time;
            self
        }

        pub fn elapsed(mut self, elapsed: Duration) -> ProbeBuilder {
            self.probe.elapsed = elapsed;
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
        let mut info = create_info();
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
            UserInput {
                url: "example.com".to_owned(),
                port: 443,
                timeout: Some(Duration::seconds(1)),
                probes_count: None,
                interval_between_probes: Duration::seconds(1),
            },
            IpAddr::from_str("93.184.216.34")?,
        );
        probes.iter().for_each(|probe| info.track(probe));
        Ok(info)
    }

    fn create_info() -> Info {
        create_info_from_probes(&[]).unwrap()
    }

    #[test]
    fn test_last_succ_and_fail_single_succ() -> Result<(), AddrParseError> {
        let probes = [success()];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.last_succ_probe, Some(probes[0].start));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_single_fail() -> Result<(), AddrParseError> {
        let probes = [failure()];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[0].start));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_succ() -> Result<(), AddrParseError> {
        let time = Utc::now();
        let second = Duration::seconds(1);
        let probes = [
            ProbeBuilder::new().start(time + second).build(),
            ProbeBuilder::new().start(time + second * 2).build(),
            ProbeBuilder::new().start(time + second * 3).build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.last_succ_probe, Some(probes[2].start));
        assert_eq!(info.last_fail_probe, None);
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mult_fail() -> Result<(), AddrParseError> {
        let time = Utc::now();
        let second = Duration::seconds(1);
        let probes = [
            ProbeBuilder::new()
                .start(time + second)
                .err(dummy_error())
                .build(),
            ProbeBuilder::new()
                .start(time + second * 2)
                .err(dummy_error())
                .build(),
            ProbeBuilder::new()
                .start(time + second * 3)
                .err(dummy_error())
                .build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.last_succ_probe, None);
        assert_eq!(info.last_fail_probe, Some(probes[2].start));
        Ok(())
    }

    #[test]
    fn test_last_succ_and_fail_mixed() -> Result<(), AddrParseError> {
        let time = Utc::now();
        let second = Duration::seconds(1);
        let probes = [
            ProbeBuilder::new()
                .start(time + second)
                .err(dummy_error())
                .build(),
            ProbeBuilder::new().start(time + second * 2).build(),
            ProbeBuilder::new()
                .start(time + second * 3)
                .err(dummy_error())
                .build(),
            ProbeBuilder::new().start(time + second * 4).build(),
            ProbeBuilder::new()
                .start(time + second * 5)
                .err(dummy_error())
                .build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.last_succ_probe, Some(probes[3].start));
        assert_eq!(info.last_fail_probe, Some(probes[4].start));
        Ok(())
    }

    #[test]
    fn test_total_uptime_downtime() -> Result<(), AddrParseError> {
        let probes = [
            ProbeBuilder::new()
                .cycle_duration(Duration::seconds(2))
                .build(),
            ProbeBuilder::new()
                .cycle_duration(Duration::seconds(3))
                .build(),
            ProbeBuilder::new()
                .cycle_duration(Duration::seconds(2))
                .err(dummy_error())
                .build(),
            ProbeBuilder::new()
                .cycle_duration(Duration::seconds(5))
                .build(),
            ProbeBuilder::new()
                .cycle_duration(Duration::seconds(20))
                .err(dummy_error())
                .build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.total_uptime, Duration::seconds(10));
        assert_eq!(info.total_downtime, Duration::seconds(22));
        Ok(())
    }

    #[test]
    fn test_min_max_sum() -> Result<(), AddrParseError> {
        let probes = [
            ProbeBuilder::new().elapsed(Duration::seconds(3)).build(),
            ProbeBuilder::new().elapsed(Duration::seconds(1)).build(),
            ProbeBuilder::new()
                .elapsed(Duration::seconds(20))
                .err(dummy_error())
                .build(),
            ProbeBuilder::new().elapsed(Duration::seconds(2)).build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.min_rtt, Duration::seconds(1));
        assert_eq!(info.max_rtt, Duration::seconds(3));
        assert_eq!(info.sum_rtt, Duration::seconds(6));
        Ok(())
    }

    #[test]
    fn test_start_end_time() -> Result<(), AddrParseError> {
        let time = Utc::now();
        let second = Duration::seconds(1);
        let probes = [
            ProbeBuilder::new().start(time + second).build(),
            ProbeBuilder::new().start(time + second * 2).build(),
            ProbeBuilder::new().start(time + second * 3).build(),
            ProbeBuilder::new()
                .start(time + second * 4)
                .start(Utc::now())
                .elapsed(Duration::seconds(3))
                .build(),
        ];
        let info = create_info_from_probes(&probes)?;
        assert_eq!(info.start_time.unwrap(), probes[0].start);
        assert_eq!(info.end_time.unwrap(), probes[3].start + probes[3].elapsed);
        Ok(())
    }
}
