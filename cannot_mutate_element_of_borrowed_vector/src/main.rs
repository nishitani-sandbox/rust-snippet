#[derive(Debug)]
struct Test(String);

fn compile_error_should_be_occurred(l: &Vec<Test>) {
    let mut first = l[0];
    first.0 = String::from("bar");
}

fn main() {
    let list = vec![Test(String::from("hoge")), Test(String::from("foo"))];
    compile_error_should_be_occurred(&list);
    println!("{:?}", list[0]);
}
