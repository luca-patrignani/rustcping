use std::{
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};
fn main() {
    let addr = SocketAddr::from(([93, 184, 216, 34], 443)); // example.com port 443
    loop {
        let start = Instant::now();
        let conn_res = TcpStream::connect_timeout(&addr, Duration::from_secs(1));
        let elapsed = start.elapsed();
        let err: Option<std::io::Error> = conn_res.err();
        println!("{}, {}", err.is_some(), elapsed.as_millis());
        thread::sleep(Duration::from_secs(1) - elapsed)
    }
}
