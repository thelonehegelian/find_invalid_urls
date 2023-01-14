use error_chain::error_chain;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{Position, Url};

error_chain! {
  foreign_links {
      ReqError(reqwest::Error);
      IoError(std::io::Error);
      UrlParseError(url::ParseError);
      JoinError(tokio::task::JoinError);
  }
}

fn main() {
    println!("Hello, world!");
}
