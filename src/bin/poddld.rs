use std::env;
extern crate reqwest;
use xml::{Event, Parser};
use curl::multi;

fn download_file(url: &String) {
  let filename = url.rsplit('/').next().unwrap();
  println!("fetching: {:?} into {:?}", url, filename);
}

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

  let mut p = Parser::new();
  p.feed_str(&feed);
  for event in p {
    match event.unwrap() {
      Event::ElementStart(tag) => {
        if tag.name == "enclosure" {
          let key: (String, Option<String>) = ("url".to_string(), None);
          let url = tag.attributes.get(&key);
          if url != None {
            download_file(url.unwrap());
          }
        }
      }
      _ => (),
    }
  }
}
