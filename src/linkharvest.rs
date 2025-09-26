use mwapi::Client;
use std::error;
use serde_json::Value;
async fn login(wiki_url: String, bot_user: String, bot_pass: String) -> Result<Client,mwapi::Error> {
    let client = mwapi::Client::builder(wiki_url.as_str())
        .set_user_agent("wikiscrape/0.1 (sylvestria.h@gmail.com)")
        .set_botpassword(bot_user.as_str(), bot_pass.as_str())
        .build().await?;

    Ok(client)
}

async fn list_pages(client: &Client) -> Result<Vec<String>,mwapi::Error> {
    let pageslist = client.get_value(&[
        ("action", "query"),
        ("list", "allpages"),
        ("aplimit", "max"),
    ]).await?;

    let titles: Vec<String> = pageslist["query"]["allpages"]
        .as_array().unwrap_or(&vec![])
        .iter()
        .filter_map(|p| p["title"].as_str().map(String::from))
        .collect();

    Ok(titles)
}

async fn external_links(client: &Client, page_titles: Vec<String>) -> Result<Vec<String>,mwapi::Error> {
    let mut all_links = Vec::new();

    for page_title in page_titles {
        let response = client.get_value(&[
            ("action", "query"),
            ("prop", "extlinks"),
            ("titles", &page_title),
        ]).await?;
        all_links.extend(page_links(&response));
    }
    Ok(all_links)
}

fn page_links(response: &Value) -> Vec<String> {
    response["query"]["pages"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|page| page.get("extlinks"))
        .filter_map(|extlinks| extlinks.as_array())
        .flatten()
        .filter_map(|link| link.get("url").and_then(|s| s.as_str()))
        .map(|s| s.trim().to_string())
        .collect()
}

pub async fn run(wiki_url: String, bot_user: String, bot_pass: String) -> Result<Vec<String>, Box<dyn error::Error>> {
    let client = login(wiki_url, bot_user, bot_pass).await?;
    let pages = list_pages(&client).await?;
    let links = external_links(&client, pages).await?;
    println!("total external links: {:?}", links.len());
    Ok(links)
}