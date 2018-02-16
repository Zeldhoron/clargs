extern crate clargs;

#[test]
fn alias_test1() {
    let mut parser = clargs::Parser::new();

    parser.add_flag(String::from("flag"));
    parser.add_alias(String::from("f"), String::from("flag"));

    parser.add_named_param::<i32>(String::from("quant"));
    parser.add_alias(String::from("q"), String::from("quant"));
    parser.add_alias(String::from("qua"), String::from("quant"));


    let args = ["", "-fq10"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert!(results.flag("flag"));
    assert_eq!(*results.named_param::<i32>("quant").unwrap(), 10);
    
    let args = ["", "--qua", "10"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(*results.named_param::<i32>("quant").unwrap(), 10);
    
    let args = ["", "--qua=10"];
    let results = parser.parse(args.iter()).ok().unwrap();
    assert_eq!(*results.named_param::<i32>("quant").unwrap(), 10);
}

#[test]
#[should_panic]
fn alias_test2() {
    let mut parser = clargs::Parser::new();
    parser.add_flag(String::from("flag1"));
    parser.add_flag(String::from("flag2"));
    parser.add_alias(String::from("f"), String::from("flag1"));
    parser.add_alias(String::from("f"), String::from("flag2"));
}

#[test]
#[should_panic]
fn alias_test3() {
    let mut parser = clargs::Parser::new();
    parser.add_alias(String::from("f"), String::from("flag"));
}
