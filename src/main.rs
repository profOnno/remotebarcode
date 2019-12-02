// This example shows how to create a basic router that maps url to different handlers.
// If you're looking for real routing middleware, check https://github.com/iron/router

extern crate iron;
extern crate keybd_event;
extern crate persistent;

use std::collections::HashMap;
use keybd_event::KeyboardKey::*;
use keybd_event::KeyBondingInstance;
use persistent::Write;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::typemap::Key;

struct Router {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<dyn Handler>>,
    //cnt: i32
}

//static mut kb:<KeyBondingInstance>=None;

#[derive(Copy, Clone)]
pub struct pers_prop;

impl Key for pers_prop { type Value = usize; }

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
 //           cnt: 10
        }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
    where
        H: Handler,
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

// TODO add error checking for weird stuff
// TODO add urldecode;
fn parse_query_to_dict(q: &str) -> HashMap<&str,&str> {
  let mut res: HashMap<&str, &str> = HashMap::new();

  for pair in q.split("&") {
    let key_value: Vec<&str> = pair.split("=").collect();
    // put result in dictionairy 
    // skip bloeh when bla=blie=bloeh
    res.insert(key_value[0], key_value[1]);
  }
  res
}

fn hello(_: &mut Request) -> IronResult<Response> {
  println!("/");
//  Ok(Response::with((status::Ok, "Hello World!")))
  Ok(Response::with((status::Ok, "Hello world from fun!")))
}

fn main() {
    let mut router = Router::new();
    let mut kb = KeyBondingInstance::new().unwrap();
   /* 
    let mut chain = Chain::new(|_: &mut Request| {
        println!("/");
        Ok(Response::with((status::Ok, "Hello world !")))
    }).link(Write::<pers_prop>::both(0));
*/
  
    kb = KeyBondingInstance::new().unwrap();
    kb.has_shift(true);
    kb.add_keys(&[KeyA, KeyZ]);
    kb.launching();

    router.add_route("".to_string(), hello);
/*
    router.add_route("".to_string(), |_: &mut Request| {
        println!("/");
//        println!("cnt:{}", router.cnt);
        Ok(Response::with((status::Ok, "Hello world !")))
    });//.link(Write::<pers_prop>::both(0));
*/

    router.add_route("hello".to_string(), |request: &mut Request| {

        let keyval = parse_query_to_dict(request.url.query().unwrap()); // TODO do match
        // exec command
        //let command = keyval.get("command");
      
        match keyval.get("code") {
          None => println!("no code given"),
          Some(v) => {
            println!("got code: {}", v);
            
          }
        }
        println!("{:?}", keyval ); 
        //println!("{:?}", command ); 
        Ok(Response::with((status::Ok, "Hello again !")))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
        println!("error");
        Ok(Response::with(status::BadRequest))
    });

    println!("Running on http://localhost:8080");
    Iron::new(router).http("localhost:8080").unwrap();
}
