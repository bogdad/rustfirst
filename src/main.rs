extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn main() {
    Server::http(hello).listen("127.0.0.1:3001").unwrap();
}

fn hello(_: Request, res: Response<Fresh>) {
    let mut res = res.start().unwrap();
    res.write_all(b"Hello World!").unwrap();
    res.end().unwrap();
}
