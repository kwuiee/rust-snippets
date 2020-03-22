extern crate crabler;

use crabler::*;

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html("a[href]", print_handler)]
struct Scraper {}

impl Scraper {
    async fn response_handler(&self, response: Response) -> Result<()> {
        println!("Status {}", response.status);
        Ok(())
    }

    async fn print_handler(&self, response: Response, a: Element) -> Result<()> {
        if let Some(href) = a.attr("href") {
            println!("Found link {} on {}", href, response.url);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let scraper = Scraper {};

    // Run scraper starting from given url and using 20 worker threads
    scraper
        .run(
            Opts::new()
                .with_urls(vec!["https://www.baidu.com/"])
                .with_threads(2),
        )
        .await
}
