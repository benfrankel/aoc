pub trait StrExt {
    fn split2<'a>(&'a self, delimiter: &str) -> (String, String);
}

impl StrExt for str {
    fn split2<'a>(&'a self, delimeter: &str) -> (String, String) {
        let mut split = self.splitn(2, delimeter);
        (
            split.next().unwrap().to_string(),
            split.next().unwrap().to_string(),
        )
    }
}
