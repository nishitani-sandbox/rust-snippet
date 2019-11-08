struct Test(u8);

impl Test {
    fn double(&self) -> u8 {
        return self.0 * 2;
    }
    fn change(self, i: u8) {
        self.0 = i;
    }
}

fn handle(list: Vec<Test>) {
    for v in list {
        v.change(0);
    }
    for v in list {
        println!("{}", v.double());
    }
}

fn main() {
    let list = vec![Test(1), Test(2), Test(3), Test(4)];
    handle(list)
}
