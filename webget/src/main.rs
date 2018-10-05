extern crate clap;
extern crate reqwest;

use std::iter::FromIterator;
use std::thread;
use clap::{Arg, App};

fn main() {
  let args = App::new("webget")
    .version("1.0")
    .author("Desert Code Camp 2018")
    .about("Runs get on a url and prints the response")
    .arg(Arg::with_name("URL")
      .required(true)
      .index(1)
      .multiple(true))
    .get_matches();

  let handles = Vec::from_iter(
    args.values_of("URL").unwrap().map(|u| {
      let url = String::from(u);
      thread::spawn(move || {
        reqwest::get(&url)
      })
    })
  );
  
  for handle in handles {
    let mut result = handle.join().unwrap();
    match result {
      Ok(mut r) => {
        println!("response = {:?}", r);
        println!("body = {:?}", r.text());
      },
      Err(e) => {
        println!("Failed to get {:?}", e);
      }
    }
  }
}
