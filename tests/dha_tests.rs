extern crate clargs;

#[test]
fn dha_test1() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);

    let args = ["clargs", "--param=value"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::UnrecognizedOption(String::from("param")));
}

#[test]
fn dha_test2() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.add_flag(String::from("flag"));

    let args = ["clargs", "--flag=value"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AssignmentToFlag(String::from("flag")));
}

#[test]
fn dha_test3() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.add_flag(String::from("flag"));
    config.add_alias(String::from("alias"), String::from("flag"));

    let args = ["clargs", "--alias=value"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AssignmentToFlagAlias(String::from("flag"), String::from("alias")));
}

#[test]
fn dha_test4() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.add_param(String::from("param"), true);

    let args = ["clargs"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::MissingRequiredParameters(vec![String::from("param")]));
}

#[test]
fn dha_test5() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.set_parameter_duplication(false);
    config.add_param(String::from("param"), false);

    let args = ["clargs", "--param=value1", "--param=value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplication(String::from("param")));
}

#[test]
fn dha_test6() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.set_parameter_duplication(false);
    config.add_param(String::from("param"), false);
    config.add_alias(String::from("alias"), String::from("param"));

    let args = ["clargs", "--param=value1", "--alias=value2"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::ParameterDuplicationAlias(String::from("param"), String::from("alias")));
}

#[test]
fn dha_test7() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.set_option_completion(true);
    config.add_param(String::from("param1"), false);
    config.add_param(String::from("param2"), false);

    let args = ["clargs", "--par=value"];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).err().unwrap();
    assert_eq!(result, clargs::ParsingError::AmbiguousOption(String::from("par"), vec![String::from("param1"), String::from("param2")]));
}

#[test]
fn dha_test8() {
    let mut config = clargs::ParsingConfig::new_all_disabled();
    config.set_double_hyphen_assignment_syntax(true);
    config.set_parameter_duplication(true);
    config.set_option_completion(true);

    config.add_param(String::from("req1"), true);
    config.add_param(String::from("req2"), true);
    config.add_alias(String::from("req2_alias"), String::from("req2"));

    config.add_param(String::from("par1"), false);
    config.add_param(String::from("par2"), false);
    config.add_param(String::from("par3"), false);
    config.add_param(String::from("par4"), false);
    config.add_param(String::from("par5"), false);
    config.add_alias(String::from("par3_alias"), String::from("par3"));
    config.add_alias(String::from("par5_alias"), String::from("par5"));

    config.add_param(String::from("ab"), false);
    config.add_param(String::from("abcd"), false);
    config.add_param(String::from("abce"), false);
    config.add_param(String::from("completion"), false);


    let args = [
        "clargs",
        "--req1=value_req1", "--req2_alias=value_req2",
        "--par1=value_par1", "--par3_alias=value_par3",
        "--par4=value_par4-1", "--par4=value_par4-2", "--par5=value_par5-1", "--par5_alias=value_par5-2",
        "--ab=value_ab", "--abcd=value_abcd",
        "--comp=value_completion"
    ];
    let result = clargs::parse(args.iter().map(|x| x.to_string()), &config).unwrap();

    assert_eq!(result.name(), "clargs");
    assert_eq!(result.get_param("req1"), Some("value_req1"));
    assert_eq!(result.get_param("req2"), Some("value_req2"));
    assert_eq!(result.get_param("par1"), Some("value_par1"));
    assert_eq!(result.get_param("par2"), None);
    assert_eq!(result.get_param("par3"), Some("value_par3"));
    assert_eq!(result.get_param("par4"), Some("value_par4-2"));
    assert_eq!(result.get_param("par5"), Some("value_par5-2"));
    assert_eq!(result.get_param("ab"), Some("value_ab"));
    assert_eq!(result.get_param("abcd"), Some("value_abcd"));
    assert_eq!(result.get_param("abce"), None);
    assert_eq!(result.get_param("completion"), Some("value_completion"));
}
