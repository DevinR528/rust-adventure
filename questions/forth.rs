// For a bit more of a challange this example is from the rust book so
// if you need some pointers https://doc.rust-lang.org/1.30.0/book/2018-edition/ch19-02-advanced-lifetimes.html
// 

struct Input<'s>(&'s str);
struct Parser<'p, 's: 'p> {
    input: &'p Input<'s>
}
impl<'p, 's: 'p> Parser<'p, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        if !self.input.0.is_ascii() {
            Err(&self.input.0[..3])
        } else {
            Ok(())
        }
    }
}
fn parse_tokens(input: Input) -> Result<(), &str> {
    Parser { input: &input }.parse()
}
