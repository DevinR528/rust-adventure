// For a bit more of a challenge this example is from the rust book so
// if you need some pointers https://doc.rust-lang.org/1.30.0/book/2018-edition/ch19-02-advanced-lifetimes.html
// It helps me to think through the flow of the thing you are trying to put a lifetime on.

struct Input<>(&str);
struct Parser<> {
    input: &Input<>
}
impl<> Parser<> {
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

fn main() {}
