// https://users.rust-lang.org/t/lifetime-problem-with-curried-function/34146
// A rarely used feature of rusts lifetime system is `impl for<'a> Foo`
// almost always used with functions. The link below is somewhat helpful
// https://doc.rust-lang.org/beta/nomicon/hrtb.html there just isn't a lot
// of info.

// Here is an example
trait SomeTrait<'a> {
    fn call_me(x: &'a str) -> Self;
}
impl<'a> SomeTrait<'a> for () {
    fn call_me(x: &'a str) -> Self {
        println!("{}", x);
        ()
    }
}
fn fooo<'a>(x: &'a str) -> impl for<'b> SomeTrait<'b> {
    let res: () = SomeTrait::call_me("helo");
    res
}

// NOW FOR THE PROBLEM
fn foo(a: &str, b: &str) -> &str {
    print!("{}", a);
    b
}

fn bar() -> impl Fn(&str) -> &str {
    move |b| foo("hey", b)
}


#[test]
fn main2() {
    let func = bar();
    println!("{}", func(" you"));
}
