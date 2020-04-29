mod helper;

#[cfg(test)]
mod integration_test {
    use std::net::Shutdown;
    use std::io::{Read, Write};

    use super::helper::server;

    #[test]
    fn test_addr() {
        let serve = server::serve();
        let addr = serve.addr.to_string();
        let _ = serve.shutdown();

        let v = addr.splitn(2, ':').collect::<Vec<&str>>();
        if let [host, port] = v.as_slice() {
            assert_eq!(&"127.0.0.1", host);
            assert!((1024..65535).contains(&port.parse::<i32>().unwrap()));
        } else {
            panic!("invalid addr");
        }
    }

    #[test]
    fn test_status_index_html() {
        let serve = server::serve();
        let mut req = server::conn(&serve.addr);
        let msg = format!(
            "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            serve.addr.to_string()
        );
        assert!(req.write_all(msg.as_bytes()).is_ok());

        let mut res = String::new();
        assert!(req.read_to_string(&mut res).is_ok());
        assert!(res.starts_with("HTTP/1.1 200 OK\r\n"));

        let _ = req.shutdown(Shutdown::Write);
        let _ = serve.shutdown();
    }
}
