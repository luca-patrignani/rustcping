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
