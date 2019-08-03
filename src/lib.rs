use std::io::Read;

pub enum SongPart{
    Text,
    Chord,
    Directive,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
