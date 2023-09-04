#[cfg(test)]
mod parse {
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
        assert_eq!(Some(3), parse(["EXEC_NAME", "1.2.3.4", "--count", "3"]).probes_count)
    }

    #[test]
    fn test_probes_count_none() {
        assert_eq!(None, parse(["EXEC_NAME", "1.2.3.4"]).probes_count)
    }

    #[test]
    fn test_probes_count_none_zero() {
        assert_eq!(None, parse(["EXEC_NAME", "1.2.3.4", "--count", "0"]).probes_count)
    }
}

#[cfg(test)]
mod info {
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
                interval_between_probes: Duration::seconds(1)
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

#[cfg(test)]
mod tcping {
    use std::{sync::mpsc::channel, thread};

    use chrono::Duration;

    use crate::{tcping, pinger::Pinger, user_input::UserInputBuilder};

    #[test]
    fn test_probes_count() {
        const PROBES_COUNT: u128 = 1000;
        let (probe_sx, probe_rx) = channel();
        let (_, closer_rx) = channel();
        struct MockPinger;
        impl Pinger for MockPinger {
            fn ping(&self) -> Option<std::io::Error> {
                None
            }
        }
        let user_input = UserInputBuilder::new("1.2.3.4".to_owned(), 443)
            .probes_count(PROBES_COUNT)
            .interval_between_probes(Duration::zero())
            .build();
        tcping(probe_sx, closer_rx, &MockPinger, user_input);
        thread::spawn(move || {
            let mut i = 0;
            while probe_rx.try_recv().is_ok() {
                i += 1;
            }
            assert_eq!(i, PROBES_COUNT);
        }).join().expect("join failed");
    }
}
