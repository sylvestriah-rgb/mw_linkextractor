use chrono::Utc;
use toml;
use serde::Deserialize;

mod linkharvest;
mod writer;
mod archiver;

#[derive(Deserialize)]
struct Config {
    wiki_api_url: String,
    bot_username: String,
    bot_password: String,
    blacklist: Vec<String>,
    successfully_archived: String
}

fn load_config() -> Result<Config,Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let date = Utc::now().format("%d%m%Y");
    let links_file = format!("extlinks_{}.txt", date);
    let todo_file = format!("todo_{}.txt", date);
    let success_file = config.successfully_archived;
    let fail_file = format!("faillinks_{}.txt", date);
    
    println!("outputs will be stored in {} {} {} {}", links_file, success_file, fail_file, todo_file);
    println!("files may be overwritten!!");

    let linklist = linkharvest::run(config.wiki_api_url, config.bot_username, config.bot_password).await?;
    let filtered_list = writer::exclude_keywords(config.blacklist,linklist);

    writer::write_to_file(filtered_list,links_file.as_str()).await?;
    writer::exclude_successful(links_file.as_str(), success_file.as_str(), todo_file.as_str())?;
    archiver::archive_org_submit(todo_file.as_str(), success_file.as_str(), fail_file.as_str()).await?;
    Ok(())
}