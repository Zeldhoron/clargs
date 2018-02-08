extern crate clargs;

#[test]
fn sh_test() {
    let mut parser = clargs::Parser::new();

    parser.add_flag(String::from("f"));

    parser.add_named_param::<i32>(String::from("n"));
    parser.add_named_param::<String>(String::from("s"));


    let args = ["", "-q"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::Unrecognized(String::from("q")));

    let args = ["", "-fq"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::Unrecognized(String::from("q")));

    let args = ["", "-n", "21", "-n"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ParameterRepetition(String::from("n")));

    let args = ["", "-n21", "-n"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ParameterRepetition(String::from("n")));

    let args = ["", "-n"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::MissingArgument(String::from("n")));

    let args = ["", "-nf"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::MissingArgument(String::from("n")));

    let args = ["", "-nf", "21"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::MissingArgument(String::from("n")));

    let args = ["", "-nf21"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::MissingArgument(String::from("n")));

    let args = ["", "-n", "str"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ArgumentParsingError(String::from("n"), String::from("str")));

    let args = ["", "-n0str"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ArgumentParsingError(String::from("n"), String::from("0str")));

    let args = ["", "-fn0str"];
    let error = parser.parse(args.iter()).err().unwrap();
    assert_eq!(error, clargs::ParsingError::ArgumentParsingError(String::from("n"), String::from("0str")));
    

    let args = ["", "-fn", "21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert!(results.flag("f"));
    assert_eq!(*results.named_param::<i32>("n").unwrap(), 21);

    let args = ["", "-fn21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert!(results.flag("f"));
    assert_eq!(*results.named_param::<i32>("n").unwrap(), 21);

    let args = ["", "-s0fn21"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(results.named_param::<String>("s").unwrap(), "0fn21");
}
