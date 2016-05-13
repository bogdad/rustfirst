extern crate hyper;
extern crate rand;
extern crate iron;
extern crate router;


use std::io::Write;
use std::thread::JoinHandle;
use std::thread;
use std::sync::mpsc::*;
use std::sync::Arc;



use iron::prelude::*;
use iron::status;
use router::Router;


#[derive(Debug, Eq, PartialEq)]
enum NodeState {
    Follower,
    Candidate,
    Leader
}

struct Node {
    id: i32,
    state: NodeState,
    election_msec: u32
}

impl Node {
    pub fn new(id: i32) -> Node {
        Node {
            id: id,
            state: NodeState::Follower,
            election_msec: rand::random::<u32>()
        }
    }
}

fn nodeById(id: i32) -> Node {
    Node::new(id)
}

fn main() {
    let node = Node::new(1);
    let mut router = Router::new();
    router.get("/state/:id", stateHandler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn stateHandler(req: &mut Request) -> IronResult<Response> {
        let idStr = req.extensions.get::<Router>().unwrap()
            .find("id").unwrap();
        let id = idStr.parse::<i32>().unwrap();
        let node = nodeById(id);
        Ok(Response::with((status::Ok, format!("{:?}\n", node.state))))
    }
}
