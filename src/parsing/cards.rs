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

fn parse_card_line<T, P, F>(
    line: &str,
    id: u32,
    parser: fn(&str) -> IResult<&str, P>,
    constructor: F,
    card_type: &str,
) -> Result<Option<T>>
where
    F: Fn(u32, P) -> T,
{
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    match parser(trimmed) {
        Ok((_, parsed_data)) => {
            let card = constructor(id, parsed_data);
            Ok(Some(card))
        }
        Err(e) => anyhow::bail!("Failed to parse {} card '{}': {}", card_type, trimmed, e),
    }
}

fn parse_green_card_line(line: &str, id: u32) -> Result<Option<GreenCard>> {
    parse_card_line(
        line,
        id,
        parse_green_card_raw,
        |id, (name, synonyms): (String, Vec<String>)| GreenCard::new(id, name, synonyms.join(", ")),
        "green",
    )
}

fn parse_red_card_line(line: &str, id: u32) -> Result<Option<RedCard>> {
    parse_card_line(
        line,
        id,
        parse_red_card_raw,
        |id, (name, description): (String, String)| RedCard::new(id, name, description),
        "red",
    )
}

async fn parse_cards<T>(
    path: impl AsRef<Path>,
    parser: fn(&str, u32) -> Result<Option<T>>,
) -> Result<Vec<T>> {
    let bytes = fs::read(path).await?;
    let lines = bytes.split(|b| *b == b'\n');
    let mut cards = Vec::new();
    let mut id_counter = 0;

    for line in lines {
        let line = String::from_utf8_lossy(line);
        if let Some(card) = parser(&line, id_counter)? {
            cards.push(card);
            id_counter += 1;
        }
    }

    Ok(cards)
}

pub async fn parse_green_cards(path: impl AsRef<Path>) -> Result<Vec<GreenCard>> {
    parse_cards(path, parse_green_card_line).await
}

pub async fn parse_red_cards(path: impl AsRef<Path>) -> Result<Vec<RedCard>> {
    parse_cards(path, parse_red_card_line).await
}
