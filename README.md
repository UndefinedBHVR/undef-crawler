# Undef-Crawler
Undef-Crawler is a simple link-based webcrawler API written in Rust.
## Features
- Get every URL listed recursively from a starting domain.
- Get all unique URLS
- Get the amount of unique URLS

## Examples
All Requests are done through various JSON endpoints.
> Get all links starting from the DuckDuckGo webpage.
```curl
curl --location --request GET '127.0.0.1:4001/scrape' \
--header 'Content-Type: application/json' \
--data-raw '{"request": "https://duckduckgo.com/"}'
```
> Get every unique URL from Pizza hut's website
```curl
curl --location --request GET '127.0.0.1:4001/scrape/unique' \
--header 'Content-Type: application/json' \
--data-raw '{"request": "https://pizzahut.com/"}'
```
>> Count all unique links on a shopping site
```curl
curl --location --request GET '127.0.0.1:4001/scrape/unique/count' \
--header 'Content-Type: application/json' \
--data-raw '{"request": "https://webscraper.io/test-sites/e-commerce/allinone/"}'
```
## Technologies
Undef-Crawler is built in Rust, using several libraries. You can see a rough justifcation for each within the `cargo.toml` file.
The most major ones as as follows:
- [Html5Ever](https://github.com/servo/html5ever)
- [Hyper](https://github.com/hyperium/hyper)
- [Tokio](https://github.com/tokio-rs/tokio)
- [Routerify](https://github.com/routerify/Routerify)

## Notes
There are currently the major caveats with the way this system functions, currently.
- Recursion has no limit, meaning if you scrape a site like say, wikipedia, you will eventually encounter a stackoverflow.
- Requests attempts not ratelimited, meaning if a site has an anti-spam filter, you will be blocked.
- There is no support for domains using `www` or other subdomains IE: `https://example.com` would work but `https://www.example.com` would not.