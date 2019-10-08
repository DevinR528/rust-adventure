// For a bit more of a challange this example is from the rust book so
// if you need some pointers https://doc.rust-lang.org/1.30.0/book/2018-edition/ch19-02-advanced-lifetimes.html
// TODO add more

struct Input<>(&str);
struct Parser<> {
    input: &Input<>
}
impl<> Parser {
    fn parse(&self) -> Result<(), &str> {
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
