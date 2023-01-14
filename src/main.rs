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

async fn get_base_url(url: &Url, document: &Document) -> Result<Url> {
    // get  base url from document
    let base_tag_href = document
        .find(Name("base"))
        .filter_map(|n| n.attr("href"))
        .nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

async fn check_links(url: &Url) -> Result<bool> {
    // send a request to the url and get status code back
    let response = reqwest::get(url.as_str()).await?;
    let status = response.status();
    // if status is 200 return true
    if status == StatusCode::OK {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        Url::parse("https://www.businessinsider.com/guides/tech/what-is-a-404-error?r=US&IR=T")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));

    let mut links = HashSet::new();
    // collect all the links from the document
    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            if let Ok(link) = base_parser.parse(href) {
                links.insert(link);
            }
        }
    }

    // check if the links are valid using tokio spawn
    let mut handles = vec![];
    // find non valid links
    for link in links {
        handles.push(tokio::spawn(async move {
            // check if the link is valid
            let is_valid = check_links(&link).await.unwrap();
            let link = link.as_str();
            if !is_valid {
                println!("Link {:?} is not valid", link);
            }
            // else {
            //     println!("Link {:?} is invalid", link);
            // }
        }));
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
