use std::fmt::Debug;
use std::fs::{read, read_to_string};
use std::path::Path;
use std::str::{from_utf8, FromStr};
use std::time::{Duration, Instant};

use age::{decrypt, scrypt::Identity, secrecy::SecretString};
use itertools::Itertools;

pub fn ints<T: FromStr>(input: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    elements(input, |c: char| !c.is_ascii_digit() && c != '-')
}

pub fn elements<T: FromStr>(input: &str, not_pred: impl Fn(char) -> bool) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .split(not_pred)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect_vec()
}

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

pub fn read_input(path: &Path, passphrase: Option<&str>) -> (String, Duration) {
    let plaintext_exists = path.exists();
    let is_encrypted = path.with_extension("encrypted").exists();
    if !plaintext_exists && !is_encrypted {
        panic!("Neither encrypted nor plaintext exists for {path:?}");
    }

    let path = if !plaintext_exists {
        path.with_extension("encrypted")
    } else {
        path.to_path_buf()
    };
    if !plaintext_exists {
        let passphrase = passphrase.expect("Passphrase is required for decrypting input");
        let passphrase = SecretString::from(passphrase);
        let identity = Identity::new(passphrase);
        let io_start = Instant::now();
        let encrypted = read(&path).expect(&format!("While opening {path:?}"));
        let read_time = io_start.elapsed();
        let decrypted = decrypt(&identity, &encrypted).unwrap();
        let parse_start = Instant::now();
        let input = from_utf8(&decrypted).unwrap().to_owned();
        let parse_time = parse_start.elapsed();
        (input, read_time + parse_time)
    } else {
        let io_start = Instant::now();
        let input = read_to_string(&path).expect(&format!("While opening {path:?}"));
        (input, io_start.elapsed())
    }
}
