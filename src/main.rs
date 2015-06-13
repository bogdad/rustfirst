extern crate hyper;

use std::io::Write;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

#[derive(Debug)]
enum NodeState {
    Follower,
    Candidate,
    Leader
}

struct Node {
    state: NodeState
}

impl Node {
    pub fn new() -> Node {
        Node{ state: NodeState::Follower }
    } 
}


fn main() {
    let node = Node::new();
    Server::http(move |req : Request,res:Response<Fresh>| {
      let mut res = res.start().unwrap();
      let uri:String = format!("{:?}", req.uri);
      match uri.find("/status") {
        Some(_) => res.write_all(format!("Status {:?}",node.state).as_bytes()).unwrap(),
        None => res.write_all(format!("Hello {:?}", uri).as_bytes()).unwrap()
      }
      res.end().unwrap();
    }).listen("127.0.0.1:3001").unwrap();
}

