extern crate clargs;

#[test]
fn full_test() {
    let mut parser = clargs::Parser::new();

    parser.add_flag(String::from("flag"));

    parser.add_named_param::<i32>(String::from("int"));


    let args = ["path/to/bin"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.path(), "path/to/bin");

    let args = ["path/to/bin", "--flag", "--int", "21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.path(), "path/to/bin");
    assert!(results.flag("flag"));
    assert_eq!(*results.named_param::<i32>("int").unwrap(), 21);

    let args = ["path/to/bin", "--flag", "--", "--int", "21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.path(), "path/to/bin");
    assert!(results.flag("flag"));
    assert_eq!(results.unnamed_param(0), "--int");
    assert_eq!(results.unnamed_param(1), "21");
}
