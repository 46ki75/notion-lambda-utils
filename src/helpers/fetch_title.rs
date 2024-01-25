use reqwest::Error;
use scraper::{Html, Selector};

pub async fn fetch_title(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let og_title_selector = Selector::parse("meta[property='og:title']").unwrap();
    let title_selector = Selector::parse("title").unwrap();

    let og_title = document
        .select(&og_title_selector)
        .next()
        .and_then(|e| e.value().attr("content"));

    let title = match og_title {
        Some(t) => t.to_string(),
        None => document
            .select(&title_selector)
            .next()
            .map_or_else(|| "".to_string(), |e| e.inner_html()),
    };

    Ok(title)
}
