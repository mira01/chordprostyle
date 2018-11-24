use std::io::Read;

pub enum SongPart{
    Text,
    Chord,
    Directive,
}

//pub struct Lexer<'a>{
//    stream: &'a mut Read,
//    state: &'a str,
//    flushing_condition: FnMut(&str) -> bool,
//}
//
//impl<'b> Lexer<'b> {
//    fn new(stream: &'b mut Read) -> &Lexer{
//        *Lexer{
//            stream: stream,
//            state: "",
//            flushing_condition: |character: &str|{
//                if let Some(_) = Some(character){
//                    true} else {false}
//            }
//        }
//    }
//}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
