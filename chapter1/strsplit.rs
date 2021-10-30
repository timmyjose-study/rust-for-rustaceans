struct StrSplit<'s, 'p> {
    delimiter: &'p str,
    document: &'s str,
}

impl<'s, 'p> Iterator for StrSplit<'s, 'p> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, d: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &d.to_string(),
    }
    .next()
}

fn main() {}