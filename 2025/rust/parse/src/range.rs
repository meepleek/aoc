use std::{
    ops::{Range, RangeInclusive},
    str::FromStr,
};

use anyhow::Context;

pub fn parse_range<T>(text: &str) -> anyhow::Result<Range<T>>
where
    T: FromStr<Err: std::error::Error + Send + Sync + 'static>,
{
    let (from, to) = parse_range_tuple(text)?;
    Ok(from..to)
}

pub fn parse_inclusive_range<T>(text: &str) -> anyhow::Result<RangeInclusive<T>>
where
    T: FromStr<Err: std::error::Error + Send + Sync + 'static>,
{
    let (from, to) = parse_range_tuple(text)?;
    Ok(from..=to)
}

pub fn parse_range_tuple<T>(text: &str) -> anyhow::Result<(T, T)>
where
    T: FromStr<Err: std::error::Error + Send + Sync + 'static>,
{
    tracing::warn!(text);

    let (from, to) = text.split_once('-').context("valid range")?;
    Ok((from.parse::<T>()?, to.parse()?))
}
