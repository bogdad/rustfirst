extern crate hyper;
extern crate rand;

use std::io::Write;
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::*;
use std::sync::Arc;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

#[derive(Debug, Eq, PartialEq)]
enum NodeState {
    Follower,
    Candidate,
    Leader
}

struct Node {
    state: NodeState,
    election_msec: u32,
    tx: Sender<&'static String>,
    rx: Receiver<&'static String>,
    t: JoinHandle<u32>
}

static checkCandidate:String = "checkCandidate".to_string();

impl Node {
    pub fn new() -> Node {
        let (tx, rx) = channel();
        let t = thread::spawn(||{
            1
        });
        Node{
            state: NodeState::Follower,
            election_msec: rand::random::<u32>(), tx: tx.clone(), rx: rx,
            t : t}
    }

    pub fn createFollowerCandidateChecker(&self) -> JoinHandle<u32> {
        thread::spawn(||{
            thread::sleep_ms(self.election_msec);
            if (self.state != NodeState::Leader) {
                let event = "checkCandidate".to_string();
                self.tx.send(event);
            }
            1
        })
    }
    
    pub fn follower() {
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

fn tick() {
}

fn follower() {
    
}
