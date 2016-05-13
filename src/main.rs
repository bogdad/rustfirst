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
    election_msec: u32,
    tx: Sender<String>,
    rx: Receiver<String>,
    t: JoinHandle<u32>
}

static checkCandidate:String = "checkCandidate".to_string();

impl Node {
    pub fn new(id: i32) -> Node {
        let (tx, rx) = channel();
        let t = thread::spawn(||{
            1
        });
        Node{
            id: id,
            state: NodeState::Follower,
            election_msec: rand::random::<u32>(), tx: tx.clone(), rx: rx,
            t : t}
    }
}

fn nodeById(id: i32) -> Node {

}

fn actorHttp(req: Request, res: Response) {
    let mut res = res.start().unwrap();
    let uri: String = format!("{:?}", req.uri);
    let node = nodeById();
    match uri.find("/status") {
        Some(_) => res.write_all(format!("Status {:?}",node.state).as_bytes()).unwrap(),
        None => res.write_all(format!("Hello {:?}", uri).as_bytes()).unwrap()
    }
    res.end().unwrap();
}

fn main() {
    let node = Node::new(1);
    let mut router = Router::new();
    router.get("/state/:id", stateHandler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn stateHandler(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id");
        let node = nodeById(id);
        Ok(Response::with((status::Ok, node.state)))
    }
}
