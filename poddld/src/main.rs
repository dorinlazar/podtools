use std::env;
use std::str;
extern crate reqwest;
use xml::{Event, Parser};

fn main() {
  println!("Podcast downloader © 2021 Dorin Lazăr");
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: pdld <rss-feed-url>");
    return;
  }
  let feed = &args[1];
  println!("Fetching {}", feed);
  let feed = reqwest::blocking::get(feed)
    .expect("Error fetching feed")
    .text()
    .expect("Error big");

  println!("Feed data: {}", feed);

  let mut p = Parser::new();
  p.feed_str(&feed);
  let search = ("url", None);
  for event in p {
    match event.unwrap() {
      Event::ElementStart(tag) => println!("start: {} {:?}", tag.name, tag.attributes[]),
      _ => (),
    }
  }
}
