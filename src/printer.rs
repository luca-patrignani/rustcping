use std::time::Duration;

pub struct Probe {
    pub elapsed: Duration,
    pub err: Option<std::io::Error>,
}

pub trait ProbePrinter {
    fn print_probe(p: Probe);
}

pub struct StdoutPrinter {}

impl ProbePrinter for StdoutPrinter {
    fn print_probe(p: Probe) {
        println!("{} {}", p.err.is_some(), p.elapsed.as_millis());
    }
}

impl Default for StdoutPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl StdoutPrinter {
    pub fn new() -> StdoutPrinter {
        StdoutPrinter {}
    }
}
