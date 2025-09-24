use std::fs;
use std::time::Duration;
use humantime::format_duration;

pub async fn write_to_file(links: Vec<String>, filename: &str) -> std::io::Result<()> {
    let link_list = links.join("\n");
    let d = 7 * links.len() as u64;
    println!("writing {} {} {}", links.len(), "links to file:", filename);
    println!("archival will require {}", format_duration(Duration::from_secs(d)));
    fs::write(filename, link_list)
}

pub fn exclude_keywords(blacklist: Vec<String>, links: Vec<String>) -> Vec<String> {
        links.into_iter()
        .filter(|link| !blacklist.iter().any(|bad| link.contains(bad)))
        .collect()
}