// https://users.rust-lang.org/t/match-on-enum-where-a-variant-holds-a-cowed-slice/34123

use std::borrow::Cow;

#[derive(Debug)]
enum TestEnum<'a> {
    Variant1,
    Variant2(Cow<'a, [u8]>),
    Variant3,
}

#[derive(Debug)]
struct TestStruct<'a> {
    e: TestEnum<'a>,
}

impl<'a> TestStruct<'a> {
    fn into_owned(self) -> TestStruct 
    // where clause may be helpful
    {
        let Self {
            e,
        } = self;
        match e {
            TestEnum::Variant2(data) => TestStruct {
                e: TestEnum::Variant2(data.into_owned().into()),
            },
            e @ _ => TestStruct {
                e,
            },
        }
    }
}

fn main() {
    let s = TestStruct {
        e: TestEnum::Variant2((&b"Hello world!"[..]).into()),
    };
    println!("{:?}", s.into_owned());
}
