extern crate xassembler;
use xassembler::*;

fn main() {
    println!(
        "{:#?}",
        body().parse(
            br#"
struct Point {
    fn new(self, x, y) {
        self.goto(x, y)
        self
    }

    fn goto(self, x, y) {
        self.x = add(x, self.x)
        self.y = add(y, self.y)
    }
}

point = (Point()).new(test.test, 5);


fn testing() {
    println("Testing!");
}


testing = (fn() { println("testing!") })
"#
        )
    );
}
