use std::{collections::HashSet, usize};
use html5ever::interface::QualName;
use html5ever::tendril::*;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::{StartTag, TagToken};
use html5ever::tokenizer::{Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts};
use html5ever::{namespace_url, LocalName};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;

pub struct Sink {
    links: Vec<String>,
}

impl Default for Sink {
    fn default() -> Self {
        Self { links: Vec::new() }
    }
}
impl TokenSink for Sink {
    type Handle = ();
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        let link_name = QualName::new(None, namespace_url!(""), LocalName::from("href"));
        if let TagToken(tag) = token {
            if tag.kind == StartTag && tag.name.to_string() == "a" {
                let attrs = tag.attrs;
                for attr in attrs {
                    if attr.name == link_name {
                        self.links.push(format!("{}", attr.value))
                    }
                }
            }
        }
        TokenSinkResult::Continue
    }
}

#[derive(Debug)]
pub struct Crawler {
    url: String,
    links: Vec<String>,
    crawled: Vec<String>,
}
impl Crawler {
    pub fn new(mut url: String) -> Self {
        if url.ends_with('/') {
            url.pop();
        }
        Self {
            url,
            links: Vec::new(),
            crawled: Vec::new(),
        }
    }

    // Takes a URL string, and crawls the page collecting every instance of an anchor tag link.
    pub fn crawl(&mut self, url: &str) {
        // Connect to the webpage and get the body as a string
        let body = {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let uri = url.parse::<Uri>().unwrap();
            let resp = futures::executor::block_on(client.get(uri)).unwrap();
            let body_bytes = futures::executor::block_on(hyper::body::to_bytes(resp.into_body()))
                .unwrap();
            String::from_utf8_lossy(&body_bytes.to_vec()).to_string()
        };
        // Create a sink to store the links into, then put it into a tokenizer
        let sink = Sink::default();
        let mut tok = Tokenizer::new(sink, TokenizerOpts::default());
        // Create a tendril as well as push the data into a buffer queue.
        let chunk = body.to_tendril();
        let mut input = BufferQueue::new();
        input.push_back(chunk);
        // Consume the buffer queue.
        tok.feed(&mut input);
        // Tell the tokenizer we have completed the tokenization process.
        tok.end();
        // Add all links, don't check for duplicates currently.
        for link in tok.sink.links {
            self.add_url(link)
        }
    }

    pub fn remove_duplicates(&mut self) {
        let mut uniques = HashSet::new();
        self.links.retain(|e| uniques.insert(e.clone()));
    }

    // Return a reference to the link vector
    pub fn get_links(&self) -> &Vec<String> {
        &self.links
    }

    // Return a count of the links.
    pub fn get_link_count(&self) -> usize {
        self.links.len()
    }

    // Add a URL to the list.
    pub fn add_url(&mut self, mut site: String) {
        if site.starts_with('/') {
            let mut s = self.url.clone();
            s.push_str(&site);
            site = s;
            self.links.push(site.clone());
            if !self.crawled.contains(&site) {
                self.crawled.push(site.clone());
                self.crawl(&site)
            }
        } else {
            self.links.push(site)
        }
    }
}
#[cfg(test)]
mod tests {
    use tokio::{runtime::Runtime, task};
    use super::Crawler;
    //Crawl the duckduckgo about page for urls. There should be these 37 URLs listed
    #[test]
    pub fn crawl_test() {
        let rt = Runtime::new().unwrap();
        let mut crawler = Crawler::new("https://duckduckgo.com".to_owned());
        crawler = rt.block_on( async move {
            task::spawn_blocking(move || {
                crawler.crawl("https://duckduckgo.com");
                crawler
            }).await.unwrap()
        });
        assert_eq!(
            crawler.get_links(),
            &vec![
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/traffic",
                "https://duckduckgo.com/",
                "https://duckduckgo.com/hiring",
                "https://duckduckgo.com/donations",
                "https://duckduckgo.com/app",
                "https://duckduckgo.com/",
                "https://spreadprivacy.com/tag/device-privacy-tips/",
                "https://duckduckgo.com/hiring",
                "https://duckduckgo.com/",
                "https://duckduckgo.com/assets/hiring/recruit-gdpr-processing-notice-1_11_20.pdf",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/app",
                "https://duckduckgo.com/traffic",
                "https://duckduckgo.com/privacy",
                "https://duckduckgo.com/press",
                "https://spreadprivacy.com/",
                "https://twitter.com/duckduckgo",
                "https://reddit.com/r/duckduckgo",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com",
                "https://spreadprivacy.com/delete-google-search-history/",
                "https://duckduckgo.com/assets/email/DuckDuckGo-Privacy-Weekly_sample.png",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/app",
                "https://duckduckgo.com/traffic",
                "https://duckduckgo.com/privacy",
                "https://duckduckgo.com/press",
                "https://spreadprivacy.com/",
                "https://twitter.com/duckduckgo",
                "https://reddit.com/r/duckduckgo",
                "https://duckduckgo.com/about",
                "https://duckduckgo.com",
                "https://duckduckgo.com/about"
            ]
        )
    }
    //Crawl the duckduckgo about page for unique urls. There should be these 16 unique urls
    #[test]
    pub fn unique_count() {
        let rt = Runtime::new().unwrap();
        let mut crawler = Crawler::new("https://duckduckgo.com".to_owned());
        crawler = rt.block_on( async move {
            task::spawn_blocking(move || {
                crawler.crawl("https://duckduckgo.com");
                crawler
            }).await.unwrap()
        });
        crawler.remove_duplicates();
        assert_eq!(crawler.get_link_count(), 16)
    }
    //Crawl the duckduckgo about page for unique urls. There should be these 16 URLs listed
    #[test]
    pub fn unique_links() {
        let rt = Runtime::new().unwrap();
        let mut crawler = Crawler::new("https://duckduckgo.com".to_owned());
        crawler = rt.block_on( async move {
            task::spawn_blocking(move || {
                crawler.crawl("https://duckduckgo.com");
                crawler
            }).await.unwrap()
        });
        crawler.remove_duplicates();
        assert_eq!(
            crawler.get_links(),
            &vec![
                "https://duckduckgo.com/about",
                "https://duckduckgo.com/",
                "https://duckduckgo.com/traffic",
                "https://duckduckgo.com/hiring",
                "https://duckduckgo.com/donations",
                "https://duckduckgo.com/app",
                "https://spreadprivacy.com/tag/device-privacy-tips/",
                "https://duckduckgo.com/assets/hiring/recruit-gdpr-processing-notice-1_11_20.pdf",
                "https://duckduckgo.com/privacy",
                "https://duckduckgo.com/press",
                "https://spreadprivacy.com/",
                "https://twitter.com/duckduckgo",
                "https://reddit.com/r/duckduckgo",
                "https://duckduckgo.com",
                "https://spreadprivacy.com/delete-google-search-history/",
                "https://duckduckgo.com/assets/email/DuckDuckGo-Privacy-Weekly_sample.png"
            ]
        )
    }
}