extern crate xassembler;
use xassembler::{compile, Rust};

#[test]
fn comment_test() {
    assert_eq!(
        compile::<Rust>(
            r#"
// hello comments!
// hello comments!
println("Testing!") // Other comment!
// Last comment
// Last comment
"#
        )
        .unwrap(),
        compile::<Rust>(
            r#"
println("Testing!")
"#
        )
        .unwrap()
    );
}
