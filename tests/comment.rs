extern crate xassembler;

#[test]
fn comment_test() {
    assert_eq!(
        xassembler::compile(r#"
// hello comments!
// hello comments!
println("Testing!") // Other comment!
// Last comment
// Last comment
"#).unwrap(),
        xassembler::compile(r#"
println("Testing!")
"#).unwrap()    
    );
}