extern crate clargs;

#[test]
fn dh_test() {
    let mut parser = clargs::Parser::new();

    parser.add_flag(String::from("abc"));
    parser.add_flag(String::from("abcd"));
    parser.add_flag(String::from("flag"));

    parser.add_named_param::<i32>(String::from("int"));
    parser.add_named_param::<String>(String::from("some-str"));


    let args = ["", "--none"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::Unrecognized(String::from("none")));

    let args = ["", "--ab"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::Ambiguous(String::from("ab"), parser.match_options("ab")));

    let args = ["", "--int", "21", "--int"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ParameterRepetition(String::from("int")));

    let args = ["", "--int"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::MissingArgument(String::from("int")));

    let args = ["", "--int", "str"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ArgumentParsingError(String::from("int"), String::from("str")));

    let args = ["", "--int="];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ArgumentParsingError(String::from("int"), String::from("")));

    
    let args = ["", "--f", "--abc"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert!(results.flag("flag"));
    assert!(results.flag("abc"));

    let args = ["", "--int", "21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(*results.named_param::<i32>("int").unwrap(), 21);

    let args = ["", "--int=21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(*results.named_param::<i32>("int").unwrap(), 21);

    let args = ["", "--some-str", "str"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.named_param::<String>("some-str").unwrap(), "str");

    let args = ["", "--some-str=str"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.named_param::<String>("some-str").unwrap(), "str");
}
