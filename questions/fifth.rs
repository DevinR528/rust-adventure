// https://users.rust-lang.org/t/match-on-enum-where-a-variant-holds-a-cowed-slice/34123
// As we have seen before there are times you have to specify two lifetimes and
// the relationship between them. This example is a bit contrived but basically we
// are making a new Stuff in one case or passing it along in the other.
// The compiler hates match expressions because it isn't aware of the meaning 
// of the arms it sees a match as one scope. 

use std::borrow::Cow;

#[derive(Debug)]
enum Stuff<'a> {
    Small,
    Large(Cow<'a, [u8]>),
    Variant3,
}

#[derive(Debug)]
struct Thingy<'a> {
    stuff: Stuff<'a>,
}

impl<'a> Thingy<'a> {
    fn into_owned(self) -> Thingy 
    // where clause may be helpful
    {
        let Self { stuff } = self;
        match stuff {
            Stuff::Large(data) => Thingy {
                stuff: Stuff::Large(data.into_owned().into()),
            },
            // `others` is the enum here match only destructuring for Stuff::Large()
            // this allows you to pass along the empty variants.
            others @ _ => Thingy {
                others,
            },
        }
    }
}

fn main() {
    let s = Thingy {
        stuff: Stuff::Large((&b"Hello world!"[..]).into()),
    };
    println!("{:?}", s.into_owned());
}
