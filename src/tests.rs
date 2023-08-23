#[cfg(test)]
mod parse {
    use crate::user_input::parse;
    #[test]
    fn test_port() {
        assert_eq!(80, parse(["example.com", "--port", "80"]).socket.port())
    }

    #[test]
    fn test_port_default() {
        assert_eq!(443, parse(["example.com"]).socket.port())
    }
}
