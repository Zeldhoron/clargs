extern crate clargs;

#[test]
fn sh_test1() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);

    let args = ["clargs", "-f"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::UnrecognizedOption(String::from("f")));
}

#[test]
fn sh_test2() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_param(String::from("p"), true);

    let args = ["clargs"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredParameters(vec![String::from("p")]));
}

#[test]
fn sh_test3() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.set_parameter_duplication(false);
    config.add_param(String::from("p"), false);

    let args = ["clargs", "-p", "value1", "-p", "value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplication(String::from("p")));
}

#[test]
fn sh_test4() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.set_parameter_duplication(false);
    config.add_param(String::from("p"), false);
    config.add_alias(String::from("a"), String::from("p"));

    let args = ["clargs", "-p", "value1", "-a", "value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplicationAlias(String::from("p"), String::from("a")));
}

#[test]
fn sh_test5() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_param(String::from("p"), false);

    let args = ["clargs", "-pf"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::UnrecognizedOption(String::from("f")));
}

#[test]
fn sh_test6() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_flag(String::from("f"));
    config.add_param(String::from("p"), true);

    let args = ["clargs", "-f"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredParameters(vec![String::from("p")]));
}

#[test]
fn sh_test7() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.set_parameter_duplication(false);
    config.add_flag(String::from("f"));
    config.add_flag(String::from("g"));
    config.add_param(String::from("p"), false);

    let args = ["clargs", "-fp", "value1", "-gp", "value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplication(String::from("p")));
}

#[test]
fn sh_test8() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.set_parameter_duplication(false);
    config.add_flag(String::from("f"));
    config.add_flag(String::from("g"));
    config.add_param(String::from("p"), false);
    config.add_alias(String::from("a"), String::from("p"));

    let args = ["clargs", "-fp", "value1", "-ga", "value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplicationAlias(String::from("p"), String::from("a")));
}

#[test]
fn sh_test9() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_flag(String::from("f"));

    let args = ["clargs", "-f4"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AssignmentToFlag(String::from("f")));
}

#[test]
fn sh_test10() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_flag(String::from("f"));
    config.add_alias(String::from("a"), String::from("f"));

    let args = ["clargs", "-a4"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AssignmentToFlagAlias(String::from("f"), String::from("a")));
}

#[test]
fn sh_test11() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_flag(String::from("f"));
    config.add_param(String::from("p"), false);

    let args = ["clargs", "-pf"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingArgument(String::from("p")));
}

#[test]
fn sh_test12() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.add_flag(String::from("f"));
    config.add_param(String::from("p"), false);
    config.add_alias(String::from("a"), String::from("p"));

    let args = ["clargs", "-af"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingArgumentAlias(String::from("p"), String::from("a")));
}

#[test]
fn sh_test13() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_single_hyphen_syntax(true);
    config.set_parameter_duplication(true);
    config.set_parameter_stacking(true);

    config.add_param(String::from("r"), true);
    config.add_param(String::from("s"), true);
    config.add_alias(String::from("a"), String::from("s"));

    config.add_param(String::from("p"), false);
    config.add_param(String::from("q"), false);
    config.add_param(String::from("m"), false);
    config.add_param(String::from("n"), false);
    config.add_param(String::from("o"), false);
    config.add_alias(String::from("b"), String::from("m"));
    config.add_alias(String::from("c"), String::from("o"));

    config.add_flag(String::from("f"));
    config.add_flag(String::from("g"));
    config.add_flag(String::from("h"));
    config.add_flag(String::from("i"));
    config.add_flag(String::from("j"));
    config.add_alias(String::from("d"), String::from("h"));
    config.add_alias(String::from("e"), String::from("j"));
    config.add_param(String::from("k"), false);


    let args = [
        "clargs",
        "-r", "value_r", "-s", "value_s",
        "-p", "value_p", "-b", "value_m",
        "-n", "value_n-1", "-n", "value_n-2", "-o", "value_o-1", "-c", "value_o-2",
        "-f", "-d", "-i", "-i", "-j", "-e", "-k", "-g"
    ];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).unwrap();

    assert_eq!(result.name(), "clargs");
    assert_eq!(result.get_param("r"), Some("value_r"));
    assert_eq!(result.get_param("s"), Some("value_s"));
    assert_eq!(result.get_param("p"), Some("value_p"));
    assert_eq!(result.get_param("q"), None);
    assert_eq!(result.get_param("m"), Some("value_m"));
    assert_eq!(result.get_param("n"), Some("value_n-2"));
    assert_eq!(result.get_param("o"), Some("value_o-2"));
    assert!(result.has_flag("f"));
    assert!(!result.has_flag("g"));
    assert!(result.has_flag("h"));
    assert!(result.has_flag("i"));
    assert!(result.has_flag("j"));
    assert_eq!(result.get_param("k"), Some("-g"));


    let args = [
        "clargs",
        "-rs", "0value_r", "0value_s",
        "-p0value_p", "-b0value_m",
        "-fndiinjoeck", "0value_n-1", "0value_n-2", "0value_o-1", "0value_o-2", "-g"
    ];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).unwrap();

    assert_eq!(result.name(), "clargs");
    assert_eq!(result.get_param("r"), Some("0value_r"));
    assert_eq!(result.get_param("s"), Some("0value_s"));
    assert_eq!(result.get_param("p"), Some("0value_p"));
    assert_eq!(result.get_param("q"), None);
    assert_eq!(result.get_param("m"), Some("0value_m"));
    assert_eq!(result.get_param("n"), Some("0value_n-2"));
    assert_eq!(result.get_param("o"), Some("0value_o-2"));
    assert!(result.has_flag("f"));
    assert!(!result.has_flag("g"));
    assert!(result.has_flag("h"));
    assert!(result.has_flag("i"));
    assert!(result.has_flag("j"));
    assert_eq!(result.get_param("k"), Some("-g"));
}
