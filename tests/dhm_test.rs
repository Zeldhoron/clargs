extern crate clargs;

#[test]
fn dhm_test() {
    let mut parser = clargs::Parser::new();

    parser.add_flag(String::from("A"));

    parser.add_named_param::<i32>(String::from("int"));


    let args = ["", "--int", "21", "--", "-A", "--int", "22", "--"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.unnamed_param(0), "-A");
    assert_eq!(results.unnamed_param(1), "--int");
    assert_eq!(results.unnamed_param(2), "22");
    assert_eq!(results.unnamed_param(3), "--");
    assert_eq!(*results.named_param::<i32>("int").unwrap(), 21);
}
