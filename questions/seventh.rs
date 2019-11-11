// https://users.rust-lang.org/t/lifetime-problem-with-curried-function/34146
// A rarely used feature of rusts lifetime system is `impl for<'a> Fn(&'a _)`
// almost always used with functions.
fn foo<'b>(a: &'b str, b: &'b str) -> &'b str {
    print!("{}", a);
    b
}

fn bar<'a>() -> impl for<'b> Fn(&'b str) -> &'b str + 'a {
    move |b| foo("hey", b)
}
#[test]
fn main2() {
    let x = bar();
    println!("{}", x(" you"));
}
