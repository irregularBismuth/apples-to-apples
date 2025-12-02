use anyhow::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::{self, AsyncBufReadExt, BufReader};

fn parse_line(line: impl AsRef<str>) {
    todo!("parse the line with nom ")
}

pub async fn parse_cards(path: impl AsRef<Path>) -> Result<()> {
    let file = fs::File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        parse_line(line);
    }
    Ok(())
}
