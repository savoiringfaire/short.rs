use http::uri::Uri;
use std::error::Error;
use std::fmt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Debug)]
pub struct NoHostError {}

impl fmt::Display for NoHostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NoHostError {
    fn description(&self) -> &str {
        "No hostname provided in target url."
    }
}

#[derive(Debug)]
pub struct NoSchemeError {}

impl fmt::Display for NoSchemeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for NoSchemeError {
    fn description(&self) -> &str {
        "No scheme (http/https) provided in target url."
    }
}

pub struct Short {
    pub token: String,
    pub target: String
}

impl Short {
    pub fn new(target: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self{
            token: Self::generate_token(10),
            target: Self::process_target(target)?
        })
    }

    fn process_target(target: String) -> Result<String, Box<dyn Error>> {
        let uri = target.parse::<Uri>()?;

        if uri.host().is_none() {
            return Err(Box::new(NoHostError{}));
        }

        if uri.scheme_part().is_none() {
            return Err(Box::new(NoSchemeError{}));
        }

        Ok(uri.to_string())
    }

    fn generate_token(n: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .collect()
    }
}
