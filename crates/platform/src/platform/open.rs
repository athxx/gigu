use robius_open::Uri;

pub fn url(uri: &str) -> Result<(), robius_open::Error> {
    Uri::new(uri).open()
}

pub fn tel(number: &str) -> Result<(), robius_open::Error> {
    url(&format!("tel:{number}"))
}

pub fn mailto(addr: &str) -> Result<(), robius_open::Error> {
    url(&format!("mailto:{addr}"))
}
