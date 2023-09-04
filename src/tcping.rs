use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

use chrono::Utc;

use crate::{pinger::Pinger, tracker::Probe, user_input::UserInput};

pub fn tcping<P: Pinger + ?Sized>(
    probe_sx: Sender<Probe>,
    closer_rx: Receiver<()>,
    pinger: &P,
    user_input: UserInput,
) {
    let mut i = 0;
    while closer_rx.try_recv().is_err() && user_input.probes_count.map_or(true, |c| i < c) {
        let start = Utc::now();
        let err = pinger.ping();
        let elapsed = Utc::now() - start;
        if elapsed < user_input.interval_between_probes {
            thread::park_timeout(
                (user_input.interval_between_probes - elapsed)
                    .to_std()
                    .unwrap(),
            );
        }
        _ = probe_sx.send(Probe {
            elapsed,
            err,
            start,
            cycle_duration: Utc::now() - start,
        });
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc::channel, thread};

    use chrono::Duration;

    use crate::{pinger::Pinger, tcping::tcping, user_input::UserInput};

    struct UserInputBuilder {
        user_input: UserInput,
    }

    impl UserInputBuilder {
        fn new(url: String, port: u16) -> Self {
            UserInputBuilder {
                user_input: UserInput {
                    url,
                    port,
                    timeout: None,
                    probes_count: None,
                    interval_between_probes: Duration::seconds(1),
                },
            }
        }

        fn probes_count(mut self, count: u128) -> Self {
            self.user_input.probes_count = Some(count);
            self
        }

        fn interval_between_probes(mut self, interval_between_probes: Duration) -> Self {
            self.user_input.interval_between_probes = interval_between_probes;
            self
        }

        fn build(self) -> UserInput {
            self.user_input
        }
    }

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
        })
        .join()
        .expect("join failed");
    }
}
