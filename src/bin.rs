extern crate xassembler;
use xassembler::*;


fn main() {
    println!(
        "{:#?}",
        body().parse(br#"
testing.test["string"]["test"][5]
"#)
//         body().parse(br#"

// FUNCTION = fn(test, hey, jude) {
//     "testing"
// }


// hey(dude)

// "#)
    );
}