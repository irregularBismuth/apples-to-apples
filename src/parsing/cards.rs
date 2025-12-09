use crate::core::{
    cards::{GreenCard, RedCard},
    deck::{GreenDeck, RedDeck},
};
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
    id: usize,
    parser: fn(&str) -> IResult<&str, P>,
    constructor: F,
) -> Result<Option<T>>
where
    F: Fn(usize, P) -> T,
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
        Err(e) => anyhow::bail!("Failed to parse card '{}': {}", trimmed, e),
    }
}

fn parse_green_card_line(line: &str, id: usize) -> Result<Option<GreenCard>> {
    parse_card_line(
        line,
        id,
        parse_green_card_raw,
        |id, (name, synonyms): (String, Vec<String>)| GreenCard::new(id, name, synonyms.join(", ")),
    )
}

fn parse_red_card_line(line: &str, id: usize) -> Result<Option<RedCard>> {
    parse_card_line(
        line,
        id,
        parse_red_card_raw,
        |id, (name, description): (String, String)| RedCard::new(id, name, description),
    )
}

async fn parse_cards<T>(
    path: impl AsRef<Path>,
    parser: fn(&str, usize) -> Result<Option<T>>,
) -> Result<Vec<T>> {
    let bytes = fs::read(path).await?;
    let lines = bytes.split(|b| *b == b'\n');
    let result: Result<Vec<T>> = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let line = String::from_utf8_lossy(line);
            parser(&line, i)
        })
        .filter_map(|res| res.transpose())
        .collect::<Result<Vec<_>, _>>();

    result
}

pub async fn parse_green_cards(path: impl AsRef<Path>) -> Result<GreenDeck> {
    let deck = parse_cards(path, parse_green_card_line).await?;
    Ok(deck.into())
}

pub async fn parse_red_cards(path: impl AsRef<Path>) -> Result<RedDeck> {
    let deck = parse_cards(path, parse_red_card_line).await?;
    Ok(deck.into())
}
