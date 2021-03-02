use crawler::*;
use error::CrawlerError;
use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tokio::task;

use std::net::SocketAddr;
use util::*;
pub mod crawler;
pub mod error;
pub mod util;
#[derive(Serialize, Deserialize)]
pub struct Req{
    request: String
}
async fn scrape_domain(mut req: Request<Body>) -> Result<Response<Body>, CrawlerError> {
    // Convert JSON into Request IE: {0: "https://google.com"} or {0: "http://google.com"}
    let url = match parse_body::<Req>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    }.request;
    let mut crawler = Crawler::new(url.clone());
    crawler = task::spawn_blocking(move || {
        crawler.crawl(&url);
        crawler
    }).await.unwrap();
    let json = json_response(json!({"status": 200, "response": crawler.get_links()}));
    Ok(json)
}

async fn scrape_unique(mut req: Request<Body>) -> Result<Response<Body>, CrawlerError> {
    // Convert JSON into Request IE: {0: "https://google.com"} or {0: "http://google.com"}
    let url = match parse_body::<Req>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    }.request;
    let mut crawler = Crawler::new(url.clone());
    crawler = task::spawn_blocking(move || {
        crawler.crawl(&url);
        crawler
    }).await.unwrap();
    crawler.remove_duplicates();
    let json = json_response(json!({"status": 200, "response": crawler.get_links()}));
    Ok(json)
}

async fn unique_count(mut req: Request<Body>) -> Result<Response<Body>, CrawlerError> {
    // Convert JSON into Request IE: {0: "https://google.com"} or {0: "http://google.com"}
    let url = match parse_body::<Req>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    }.request;
    let mut crawler = Crawler::new(url.clone());
    crawler = task::spawn_blocking(move || {
        crawler.crawl(&url);
        crawler
    }).await.unwrap();
    crawler.remove_duplicates();
    let json = json_response(json!({"status": 200, "response": crawler.get_link_count()}));
    Ok(json)
}

fn create_router() -> Router<Body, CrawlerError> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .get("/scrape", scrape_domain)
        .get("/scrape/unique", scrape_unique)
        .get("/scrape/unique/count", unique_count)
        .build()
        .unwrap()
}

async fn logger(req: Request<Body>) -> Result<Request<Body>, CrawlerError> {
    #[cfg(not(release))]
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

#[tokio::main]
async fn main() {
    let router = create_router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 4001));
    let server = Server::bind(&addr).serve(service);
    println!("Webcrawler initialized on {}", addr);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
