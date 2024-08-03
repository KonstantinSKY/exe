use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub packages: Packages,
}

impl Config {
    pub fn new(key:&str) -> Self {
        crate::configs::get(key)
    }
}

#[derive(Deserialize, Debug)]
pub struct Packages {
    pub unneeded: String,
    pub requirements: String,
    pub requirements_2: String,
    pub internet: String,
    pub communication: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let config = Config::new("manjaro");

        eprintln!("Config: {config:#?}");

    }
}