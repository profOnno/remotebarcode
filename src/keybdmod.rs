extern crate keybd_event;

//use std::collections::HashMap;
use keybd_event::KeyboardKey;
use keybd_event::KeyboardKey::*;
use keybd_event::KeyBondingInstance;

use std::thread::sleep;
use std::time::Duration;

// TODO look if chars can be used
// also there is double conversion... KeyX has a number in keybd_event source
fn lookup(s: char) -> KeyboardKey {
  match s {
    '0' => Key0,
    '1' => Key1,
    '2' => Key2,
    '3' => Key3,
    '4' => Key4,
    '5' => Key5,
    '6' => Key6,
    '7' => Key7,
    '8' => Key8,
    '9' => Key9,
      _ => KeyKPDot,
  }
}

pub fn type_it(s: &str) {
  let mut kb = KeyBondingInstance::new().unwrap();
  
  println!("got s: {}, len: {}", s, s.len());


  #[cfg(target_os = "linux")]
  sleep(Duration::from_secs(1)); //needed?

  // doit in one run?
  // TODO expand so shift works and commands can be stripped
  for c in s.chars() {
    println!("{} => {:?}", c, lookup(c));
    kb.add_key(lookup(c));
  }

//  kb.has_shift(true);
//  kb.add_keys(&[KeyA, KeyZ]);
//  kb.launching();
//  kb.clear();
//  kb.has_shift(false);
  kb.launching();
}
