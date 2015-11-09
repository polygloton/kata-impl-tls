use std::env;
use std::io;
use std::io::prelude::*;
use std::net;

const HTTP_PORT: u16 = 80;

#[derive(Debug)]
enum ClientError {
    Io(io::Error),
    AddrParse(net::AddrParseError),
    Usage,
    BadUrl
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> ClientError {
        ClientError::Io(err)
    }
}

impl From<net::AddrParseError> for ClientError {
    fn from(err: net::AddrParseError) -> ClientError {
        ClientError::AddrParse(err)
    }
}

struct Success;

struct Url<'a> {
    host: &'a str,
    path: &'a str
}

fn parse_url(url: &str) -> Option<Url>  {
    match url.find("//") {
        None => None,
        Some(i1) => {
            let (_, prt1) = url.split_at(i1 + 2);
            match prt1.find("/") {
                None => None,
                Some(i2) => {
                    let (host, path) = prt1.split_at(i2);
                    let url_struct = Url { host: host, path: path};
                    Some(url_struct)}}}}}

fn program() -> String {
    match env::args().nth(0) {
        None => "<PROGRAM>".to_string(),
        Some(p) => p
    }
}

fn get_request(stream: &mut net::TcpStream,
               url: Url) -> Result<Success, ClientError> {
    try!(stream.write_all(format!("GET {} HTTP/1.1\r\n", url.path).as_bytes()));
    try!(stream.write_all(format!("Host: {}\r\n", url.host).as_bytes()));
    try!(stream.write_all(b"Connection: Close\r\n"));
    try!(stream.write_all(b"\r\n"));
    Ok::<Success, ClientError>(Success)
}

fn client(mut argv: env::Args) -> Result<String, ClientError> {
    let url_input = try!(argv.nth(1).ok_or(ClientError::Usage));
    let url = try!(parse_url(&url_input).ok_or(ClientError::BadUrl));
    let mut stream = try!(net::TcpStream::connect((url.host, HTTP_PORT)));
    try!(get_request(&mut stream, url));
    let mut response_body = String::new();
    try!(stream.read_to_string(&mut response_body));
    Ok(response_body)
}

fn main() {
    match client(env::args()) {
        Ok(result) => println!("{}", result),
        Err(ClientError::Usage) => println!("Usage: {} <URL>", program()),
        Err(ClientError::BadUrl) => println!("Error - malformed URL"),
        Err(err) => println!("Error: {:?}", err)
    };
}
