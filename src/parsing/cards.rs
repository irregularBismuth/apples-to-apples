use crate::core::cards::{GreenCard, RedCard};
use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char, multispace0},
    combinator::rest,
    multi::separated_list1,
    sequence::delimited,
};
use std::path::Path;
use tokio::fs;

fn parse_synonym(input: &str) -> IResult<&str, &str> {
    alt((take_until(","), take_until(")"))).parse(input)
}

fn parse_green_card_raw(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, name) = delimited(char('['), take_until("]"), char(']')).parse(input)?;
    let (input, _) = (multispace0, char('-'), multispace0).parse(input)?;
    let (input, synonyms) = delimited(
        char('('),
        separated_list1((multispace0, char(','), multispace0), parse_synonym),
        char(')'),
    )
    .parse(input)?;

    let synonyms: Vec<String> = synonyms
        .iter()
        .map(|s| String::from_utf8_lossy(s.as_bytes()).trim().to_string())
        .collect();

    Ok((input, (name.trim().to_string(), synonyms)))
}

fn parse_red_card_raw(input: &str) -> IResult<&str, (String, String)> {
    let (input, name) = delimited(char('['), take_until("]"), char(']')).parse(input)?;
    let (input, _) = (multispace0, char('-'), multispace0).parse(input)?;
    let (input, description) = rest.parse(input)?;
    Ok((
        input,
        (name.trim().to_string(), description.trim().to_string()),
    ))
}

pub fn parse_green_card_line(line: &str, id: u32) -> Result<Option<GreenCard>> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    match parse_green_card_raw(trimmed) {
        Ok((_, (name, synonyms))) => {
            let card = GreenCard::new(id, name, synonyms.join(", "));
            Ok(Some(card))
        }
        Err(e) => anyhow::bail!("Failed to parse green card '{}': {}", trimmed, e),
    }
}

pub fn parse_red_card_line(line: &str, id: u32) -> Result<Option<RedCard>> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    match parse_red_card_raw(trimmed) {
        Ok((_, (name, description))) => {
            let card = RedCard::new(id, name, description);
            Ok(Some(card))
        }
        Err(e) => anyhow::bail!("Failed to parse red card '{}': {}", trimmed, e),
    }
}

pub async fn parse_green_cards(path: impl AsRef<Path>) -> Result<Vec<GreenCard>> {
    let bytes = fs::read(path).await?;
    let lines = bytes.split(|b| *b == b'\n');
    let mut cards = Vec::new();
    let mut id_counter = 1u32;

    for line in lines {
        let line = String::from_utf8_lossy(line);
        if let Some(card) = parse_green_card_line(&line, id_counter)? {
            cards.push(card);
            id_counter += 1;
        }
    }

    Ok(cards)
}

pub async fn parse_red_cards(path: impl AsRef<Path>) -> Result<Vec<RedCard>> {
    let bytes = fs::read(path).await?;
    let lines = bytes.split(|b| *b == b'\n');
    let mut cards = Vec::new();
    let mut id_counter = 1u32;

    for line in lines {
        let line = String::from_utf8_lossy(line);
        if let Some(card) = parse_red_card_line(&line, id_counter)? {
            cards.push(card);
            id_counter += 1;
        }
    }

    Ok(cards)
}
