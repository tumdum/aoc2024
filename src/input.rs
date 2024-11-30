use std::fmt::Debug;
use std::str::FromStr;

pub fn tokens<T>(input: &str, sep: Option<&str>) -> Vec<T>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    if let Some(sep) = sep {
        input
            .split(sep)
            .filter(|v| !v.is_empty())
            .flat_map(|v| v.parse().ok())
            .collect()
    } else {
        input
            .split_whitespace()
            .flat_map(|v| v.parse().ok())
            .collect()
    }
}

pub fn token_groups<T>(input: &str, sep: &str, inner_sep: Option<&str>) -> Vec<Vec<T>>
where
    T: FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    input
        .split(sep)
        .filter(|l| !l.is_empty())
        .map(|sub| tokens(sub, inner_sep))
        .collect()
}
