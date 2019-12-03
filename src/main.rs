// This example shows how to create a basic router that maps url to different handlers.
// If you're looking for real routing middleware, check https://github.com/iron/router

extern crate iron;
extern crate keybd_event;
extern crate persistent;

use std::collections::HashMap;
use keybd_event::KeyboardKey::*;
use keybd_event::KeyBondingInstance;
use persistent::{Write,Read};
use std::thread::sleep;
use std::time::Duration;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::typemap::Key;
mod keybdmod;
use keybdmod::*;

struct Router {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<dyn Handler>>,
    //cnt: i32
}

//static mut kb:<KeyBondingInstance>=None;

#[derive(Copy, Clone)]
pub struct HitCounter;
impl Key for HitCounter { type Value = usize; }

#[derive(Copy, Clone)]
pub struct RouteProperties;
//impl Key for RouteProperties { type Value = KeyBondingInstance; }
//impl Key for RouteProperties { type Value = String; }
impl Key for RouteProperties { type Value = Mytype; }

pub struct Mytype {
  m: u32,
  s: String
//  k: KeyBondingInstance 
// not thread safe!?
}

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

fn hello(req: &mut Request) -> IronResult<Response> {
  let mutex = req.get::<Write<HitCounter>>().unwrap();
  let mut count = mutex.lock().unwrap();
  println!("/");
  println!("{}", *count);
  *count +=1;
//  Ok(Response::with((status::Ok, "Hello World!")))
  Ok(Response::with((status::Ok, "Hello world from fun!")))
}

fn hello3(req: &mut Request) -> IronResult<Response> {
  let arc = req.get::<Read<RouteProperties>>().unwrap();
  let props = arc.as_ref();
  println!("/");
  println!("m: {}, s: {}", props.m, props.s);

  // needs time?
//  sleep(Duration::from_secs(3));
//  println!("waited 3");
  type_it("5123");

//  Ok(Response::with((status::Ok, "Hello World!")))
  Ok(Response::with((status::Ok, "Hello world from fun!")))
}


fn main() {
    let mut router = Router::new();
//    type_az(1);
   /* 
    let mut chain = Chain::new(|_: &mut Request| {
        println!("/");
        Ok(Response::with((status::Ok, "Hello world !")))
    }).link(Write::<PersProp>::both(0));
*/
 
    //let kb = KeyBondingInstance::new().unwrap();
/*
    let kb = match KeyBondingInstance::new() {
      Ok(v) => { println!("got bonding"); Ok(v) },
      Err(e) => { 
          println!("failure: {}", e);
          Err(e)
        }, 
    };

    let mut kb = kb.unwrap();
*/



    router.add_route("".to_string(), hello);
/*
    router.add_route("".to_string(), |_: &mut Request| {
        println!("/");
//        println!("cnt:{}", router.cnt);
        Ok(Response::with((status::Ok, "Hello world !")))
    });//.link(Write::<PersProp>::both(0));
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


    //let mut chain = Chain::new(hello);
    let mut chain = Chain::new(hello);
    chain.link(Write::<HitCounter>::both(10)); //both?? set variable

    let mut chain3 = Chain::new(hello3);
    //chain3.link(Read::<RouteProperties>::both("biebop".to_string())); //both?? set variable
    chain3.link(Read::<RouteProperties>::both(Mytype { m: 2, s: "biebop".to_string()})); //both?? set variable
//  Why doesn't this work?
//    let mut chain2 = (Chain::new(hello))
//      .link(Write::<HitCounter>::both(0));

    println!("Running on http://localhost:8080");
    //Iron::new(router).http("localhost:8080").unwrap();
    Iron::new(chain3).http("localhost:8080").unwrap();

}
