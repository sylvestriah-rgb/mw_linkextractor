use reqwest;
use std::time::Duration;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

pub async fn archive_org_submit(linksfile: &str, successname: &str, failname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let openlinks = File::open(linksfile)?;
    let reader = BufReader::new(openlinks);
    for line in reader.lines(){
        let url = line?;
        let response = reqwest::get(&format!("https://web.archive.org/save/{}", url)).await?;
        let mut successfile = OpenOptions::new().create(true).write(true).append(true).open(successname)?;
        let mut failfile = OpenOptions::new().create(true).write(true).append(true).open(failname)?;

        println!("currently archiving: {}", &url);
        if response.status().is_success() {
            writeln!(successfile, "{}", url)?;
            println!("success: {}", url);
        }

        else{
            writeln!(failfile, "{}", url)?;
            println!("failed: {}", url);
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Ok(())
}