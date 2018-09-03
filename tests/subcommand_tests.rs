extern crate clargs;

#[test]
fn subcommand_test1() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_required(true);

    let args = ["clargs"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredSubcommand);
}

#[test]
fn subcommand_test2() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_required(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(0);

    let args = ["clargs"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredSubcommand);
}

#[test]
fn subcommand_test3() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_required(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(0);

    let args = ["clargs", "hello", "world"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::UnrecognizedSubcommand(String::from("hello")));
}

#[test]
fn subcommand_test4() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_required(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(1);

    let args = ["clargs", "hello", "world"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::UnrecognizedSubcommand(String::from("world")));
}

#[test]
fn subcommand_test5() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_required(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(2);

    let args = ["clargs", "hello", "world"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredSubcommand);
}

#[test]
fn subcommand_test6() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_completion(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(0);
    config.add_subcommand(String::from("cmnd1"));
    config.add_subcommand(String::from("cmnd2"));

    let args = ["clargs", "cmnd"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AmbiguousSubcommand(String::from("cmnd"), vec![String::from("cmnd1"), String::from("cmnd2")]));
}

#[test]
fn subcommand_test7() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_subcommand_completion(true);
    config.set_subcommand_index(true);
    config.set_subcommand_index_value(2);
    config.add_subcommand(String::from("cmnd-abcde"));
    config.add_subcommand(String::from("cmnd-01234"));

    let args = ["clargs", "hello", "world", "cmnd-a", "unnamed"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).unwrap();

    assert_eq!(result.unnamed_params(), &vec![String::from("hello"), String::from("world")]);
    assert_eq!(result.get_subcommand(), Some("cmnd-abcde"));
    assert_eq!(result.subcommand_args(), &vec![String::from("cmnd-abcde"), String::from("unnamed")]);
}
