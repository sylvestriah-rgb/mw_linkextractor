use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use humantime::format_duration;
use std::collections::HashSet;
use std::fs::{File, OpenOptions, write};

pub async fn write_to_file(links: Vec<String>, filename: &str) -> std::io::Result<()> {
    let link_list = links.join("\n");
    println!("non-blacklisted links: {}", links.len());
    write(filename, link_list)
}

pub fn exclude_keywords(blacklist: Vec<String>, links: Vec<String>) -> Vec<String> {
        links.into_iter()
        .filter(|link| !blacklist.iter().any(|bad| link.contains(bad)))
        .collect()
}

pub fn exclude_successful(externalname: &str, successname: &str, todolist: &str) -> std::io::Result<()> {
    let archivedhash: HashSet<String> = std::fs::read_to_string(successname)?
        .lines().map(|s| s.to_string()).collect();
    let openlinks = File::open(externalname)?;
    let reader = BufReader::new(openlinks);
    let mut notarchived = OpenOptions::new().create(true).write(true).truncate(true).open(todolist)?;

    let mut archived = 0;
    let mut todo: i32 = 0;
    for line in reader.lines(){
        let line = line?;
        if !archivedhash.contains(&line) {
            writeln!(notarchived, "{}", line)?;
            todo = todo + 1;
        }
        else {
            archived = archived + 1;
        }
    }
    println!("excluded due to previous archival: {}", archived);
    println!("links to archive: {}", todo);
    let d = 30 * todo as u64;
    println!("archival will require {} {}", format_duration(Duration::from_secs(d)), "(assumes 30s/link)");
    Ok(())
}