use std::io;
use std::path::Path;
use std::fs;
use std::fmt;
use std::borrow::Cow;

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use rocket::request::FromParam;
use rocket::http::RawStr;

/// A _probably_ unique paste ID.
pub struct ItemID<'a>(Cow<'a, str>);

impl<'a> ItemID<'a> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize, url: bool) -> ItemID<'static> {
        let random: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .collect();

        match conflict_check(&random, url) {
            Ok(_) => ItemID(Cow::Owned(random)),
            Err(_) => ItemID::new(size, url)
        }
    }
}

impl<'a> fmt::Display for ItemID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Returns `true` if `id` is a valid paste ID and `false` otherwise.
fn valid_id(id: &str) -> bool {
    id.chars().all(|c| {
        (c >= 'a' && c <= 'z')
            || (c >= 'A' && c <= 'Z')
            || (c >= '0' && c <= '9')
    })
}

/// Returns an instance of `ItemID` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for ItemID<'a> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<ItemID<'a>, &'a RawStr> {
        let mut name = param.split(".");

        let (a, _) = name.size_hint();
        if a > 2 {
            return Err(param)
        }

        match valid_id(name.next().unwrap()) {
            true => Ok(ItemID(Cow::Borrowed(param))),
            false => Err(param)
        }
    }
}


pub(crate) fn conflict_check(name: &String, url: bool) -> io::Result<()> {
    let target = if url {
        "content/files/"
    } else {
        "content/urls/"
    };

    for i in fs::read_dir(Path::new(&target))? {
        let b = i?.file_name().into_string().unwrap();
        let entry = b.split(".").next().unwrap();

        if entry.to_string() == *name {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Conflict Found"))
        }
    }
    Ok(())
}