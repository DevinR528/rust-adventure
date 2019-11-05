// https://users.rust-lang.org/t/lifetime-problem-with-curried-function/34146
fn foo<'a, 'b>(a: &'b str, b: &'b str) -> &'b str {
    b
}

fn bar<'a>() -> impl for<'b> Fn(&'b str) -> &'b str + 'a {
    move |b| foo("hey", b)
}

fn main() {
    let x = bar();
    println!("{}", x(" yo"));
}
