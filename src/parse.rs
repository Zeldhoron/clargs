use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;

use parsing_error::ParsingError;
use parsing_config::ArgDesc;
use parsing_config::ParsingConfig;
use parsed_args::ParsedArgs;


fn match_option<'a>(config: &'a ParsingConfig, name: &str) -> Vec<(&'a String, &'a ArgDesc)> {
    let mut matches = Vec::new();
    for value in config.options.iter() {
        if value.0 == name {
            return vec![value];
        } else if value.0.starts_with(name) {
            matches.push(value);
        }
    }
    matches.sort_unstable();
    matches
}

fn match_subcmd<'a>(config: &'a ParsingConfig, name: &str) -> Vec<&'a String> {
    let mut matches = Vec::new();
    for value in config.subcmds.iter() {
        if value == name {
            return vec![value];
        } else if value.starts_with(name) {
            matches.push(value);
        }
    }
    matches.sort_unstable();
    matches
}

fn parse_option_name<'a>(config: &'a ParsingConfig, name: &'a str) -> Result<(&'a str, &'a ArgDesc), ParsingError> {
    if config.option_completion {
        let matches = match_option(config, name);
        match matches.len() {
            0 => Err(ParsingError::UnrecognizedOption(name.to_string())),
            1 => Ok((matches[0].0.as_str(), matches[0].1)),
            _ => Err(ParsingError::AmbiguousOption(name.to_string(), matches.iter().map(|x| x.0.to_string()).collect())),
        }
    } else {
        match config.options.get(name) {
            Some(desc) => Ok((name, desc)),
            None => Err(ParsingError::UnrecognizedOption(name.to_string())),
        }
    }
}

fn insert_param(params: &mut HashMap<String, String>, config: &ParsingConfig, target: &str, name: &str, value: String, aliased: bool) -> Result<(), ParsingError> {
    match params.entry(target.to_string()) {
        Entry::Vacant(spot) => {
            spot.insert(value);
            Ok(())
        },
        Entry::Occupied(mut spot) => if config.param_duplication {
            spot.insert(value);
            Ok(())
        } else {
            if aliased {
                Err(ParsingError::ParameterDuplicationAlias(target.to_string(), name.to_string()))
            } else {
                Err(ParsingError::ParameterDuplication(target.to_string()))
            }
        }
    }
}

fn resolve_target<'a>(config: &ParsingConfig, name: &'a str, desc: &'a ArgDesc) -> (&'a str, bool, bool) {
    match desc {
        ArgDesc::Flag => (name, false, true),
        ArgDesc::Param(_) => (name, false, false),
        ArgDesc::Alias(alias_target) => match config.options.get(alias_target).unwrap() {
            ArgDesc::Flag => (alias_target.as_str(), true, true),
            ArgDesc::Param(_) => (alias_target.as_str(), true, false),
            ArgDesc::Alias(_) => panic!(),
        },
    }
}


/// Parses the argument list according to the provided configuration.
/// The result is either an error or the parsed arguments.
///
/// Note that the argument list passed to this function cannot contain non-unicode characters.
pub fn parse<I: Iterator<Item=String>>(mut args: I, config: &ParsingConfig) -> Result<ParsedArgs, ParsingError> {
    let name = args.next().unwrap_or(String::new());
    let mut flags = HashSet::new();
    let mut params = HashMap::new();
    let mut unnameds = Vec::new();
    let mut subcommand = Vec::new();

    while let Some(arg) = args.next() {
        if config.dh_marker && arg == "--" {
            if config.store_dh_marker {
                unnameds.push(arg);
            }
            for arg in args {
                unnameds.push(arg);
            }
            break;
        }


        let eq_index = arg.find('=');

        if config.dha_syntax && arg.starts_with("--") && eq_index != None {
            let eq_index = eq_index.unwrap();
            let opt_name = &arg[2..eq_index];
            let value = &arg[eq_index+1..];
            let (name, desc) = parse_option_name(config, opt_name)?;
            let (target, aliased, flag) = resolve_target(config, name, desc);

            if flag {
                if aliased {
                    return Err(ParsingError::AssignmentToFlagAlias(target.to_string(), name.to_string()));
                } else {
                    return Err(ParsingError::AssignmentToFlag(target.to_string()));
                }
            } else {
                insert_param(&mut params, config, target, name, value.to_string(), aliased)?;
                continue;
            }
        }

        if config.dh_syntax && arg.starts_with("--") {
            let opt_name = &arg[2..];
            let (name, desc) = parse_option_name(config, opt_name)?;
            let (target, aliased, flag) = resolve_target(config, name, desc);

            if flag {
                flags.insert(target.to_string());
            } else {
                let value = match args.next() {
                    Some(arg) => arg,
                    None => if aliased {
                        return Err(ParsingError::MissingArgumentAlias(target.to_string(), name.to_string()));
                    } else {
                        return Err(ParsingError::MissingArgument(target.to_string()));
                    },
                };
                insert_param(&mut params, config, target, name, value, aliased)?;
            }
            continue;
        }

        if config.sh_syntax && arg.starts_with('-') {
            let mut opts = Vec::new();
            let mut value = String::new();
            let mut iter = arg.chars().skip(1);
            while let Some(arg_char) = iter.next() {
                if arg_char.is_alphabetic() {
                    opts.push(arg_char);
                } else {
                    value.push(arg_char);
                    while let Some(arg_char) = iter.next() {
                        value.push(arg_char);
                    }
                }
            }

            if opts.len() == 0 {
                let desc = match config.options.get("") {
                    Some(desc) => desc,
                    None => if value.is_empty() {
                        unnameds.push(String::from("-"));
                        continue;
                    } else {
                        return Err(ParsingError::UnrecognizedOption(value));
                    },
                };
                let (target, aliased, flag) = resolve_target(config, "", desc);

                if flag {
                    if value.is_empty() {
                        flags.insert(target.to_string());
                    } else if aliased {
                        return Err(ParsingError::AssignmentToFlagAlias(target.to_string(), String::new()));
                    } else {
                        return Err(ParsingError::AssignmentToFlag(target.to_string()));
                    }
                } else if value.is_empty() {
                    if aliased {
                        return Err(ParsingError::MissingArgumentAlias(target.to_string(), String::new()));
                    } else {
                        return Err(ParsingError::MissingArgument(target.to_string()));
                    }
                } else {
                    insert_param(&mut params, config, target, "", value, aliased)?;
                }
                continue;
            }

            let mut last_was_flag = false;
            let mut param_opts = Vec::new();
            let mut last_flag = (String::new(), false, String::new());
            for opt in opts {
                let name = opt.to_string();
                let desc = match config.options.get(&name) {
                    Some(desc) => desc,
                    None => return Err(ParsingError::UnrecognizedOption(name)),
                };
                let (target, aliased, flag) = resolve_target(config, &name, desc);

                if flag {
                    last_was_flag = true;
                    flags.insert(target.to_string());
                    last_flag = (target.to_string(), aliased, name.clone());
                } else {
                    last_was_flag = false;
                    param_opts.push((target.to_string(), aliased, name.clone()));
                }
            }

            if config.param_stacking && !param_opts.is_empty() {
                if last_was_flag && !value.is_empty() {
                    let (target, aliased, name) = last_flag;
                    if aliased {
                        return Err(ParsingError::AssignmentToFlagAlias(target, name));
                    } else {
                        return Err(ParsingError::AssignmentToFlag(target));
                    }
                }

                let mut param_vals = LinkedList::new();
                for i in 0..param_opts.len()-1 {
                    param_vals.push_back(match args.next() {
                        Some(arg) => arg,
                        None => {
                            param_opts.resize(i + 1, (String::new(), false, String::new()));
                            let (target, aliased, name) = param_opts.pop().unwrap();
                            if aliased {
                                return Err(ParsingError::MissingArgumentAlias(target, name));
                            } else {
                                return Err(ParsingError::MissingArgument(target));
                            }
                        },
                    });
                }

                if value.is_empty() {
                    param_vals.push_back(match args.next() {
                        Some(arg) => arg,
                        None => {
                            let (target, aliased, name) = param_opts.pop().unwrap();
                            if aliased {
                                return Err(ParsingError::MissingArgumentAlias(target, name));
                            } else {
                                return Err(ParsingError::MissingArgument(target));
                            }
                        },
                    });
                } else {
                    param_vals.push_back(value);
                }

                for i in 0..param_opts.len() {
                    insert_param(&mut params, config, param_opts[i].0.as_str(), param_opts[i].2.as_str(), param_vals.pop_front().unwrap(), param_opts[i].1)?;
                }
            } else if !param_opts.is_empty() {
                if last_was_flag || param_opts.len() > 1 {
                    param_opts.resize(1, (String::new(), false, String::new()));
                    let (target, aliased, name) = param_opts.pop().unwrap();
                    if aliased {
                        return Err(ParsingError::MissingArgumentAlias(target, name));
                    } else {
                        return Err(ParsingError::MissingArgument(target));
                    }
                }

                let (target, aliased, name) = param_opts.pop().unwrap();
                if value.is_empty() {
                    value = match args.next() {
                        Some(arg) => arg,
                        None => {
                            if aliased {
                                return Err(ParsingError::MissingArgumentAlias(target, name));
                            } else {
                                return Err(ParsingError::MissingArgument(target));
                            }
                        },
                    };
                }
                insert_param(&mut params, config, &target, &name, value, aliased)?;
            } else if !value.is_empty() {
                let (target, aliased, name) = last_flag;
                if aliased {
                    return Err(ParsingError::AssignmentToFlagAlias(target, name));
                } else {
                    return Err(ParsingError::AssignmentToFlag(target));
                }
            }
            continue;
        }

        if config.a_syntax && eq_index != None {
            let eq_index = eq_index.unwrap();
            let opt_name = &arg[..eq_index];
            let value = &arg[eq_index+1..];
            let (name, desc) = parse_option_name(config, opt_name)?;
            let (target, aliased, flag) = resolve_target(config, name, desc);

            if flag {
                if aliased {
                    return Err(ParsingError::AssignmentToFlagAlias(target.to_string(), name.to_string()));
                } else {
                    return Err(ParsingError::AssignmentToFlag(target.to_string()));
                }
            } else {
                insert_param(&mut params, config, target, name, value.to_string(), aliased)?;
                continue;
            }
        }


        if config.subcmd_required && config.subcmd_index {
            if unnameds.len() == config.subcmd_index_value {
                let name = if config.subcmd_completion {
                    let matches = match_subcmd(config, &arg);
                    match matches.len() {
                        0 => return Err(ParsingError::UnrecognizedSubcommand(arg)),
                        1 => matches[0],
                        _ => return Err(ParsingError::AmbiguousSubcommand(arg, matches.iter().map(|x| x.to_string()).collect())),
                    }
                } else {
                    match config.subcmds.get(&arg) {
                        Some(name) => name,
                        None => return Err(ParsingError::UnrecognizedSubcommand(arg)),
                    }
                };

                subcommand.push(name.to_string());
                while let Some(arg) = args.next() {
                    subcommand.push(arg);
                }
                break;
            }
        } else if config.subcmd_index {
            if unnameds.len() == config.subcmd_index_value {
                if config.subcmd_completion {
                    let matches = match_subcmd(config, &arg);
                    match matches.len() {
                        0 => {},
                        1 => {
                            subcommand.push(matches[0].to_string());
                            while let Some(arg) = args.next() {
                                subcommand.push(arg);
                            }
                            break;
                        },
                        _ => return Err(ParsingError::AmbiguousSubcommand(arg, matches.iter().map(|x| x.to_string()).collect())),
                    }
                } else if let Some(name ) = config.subcmds.get(&arg) {
                    subcommand.push(name.to_string());
                    while let Some(arg) = args.next() {
                        subcommand.push(arg);
                    }
                    break;
                }
            }
        } else if let Some(_) = config.subcmds.get(&arg) {
            subcommand.push(arg);
            while let Some(arg) = args.next() {
                subcommand.push(arg);
            }
            break;
        }
        
        unnameds.push(arg);
    }

    if config.subcmd_required && subcommand.len() == 0 {
        return Err(ParsingError::MissingRequiredSubcommand);
    }
    let mut required_params = Vec::new();
    for opt in config.options.iter() {
        if let (name, ArgDesc::Param(required)) = opt {
            if *required && params.get(name) == None {
                required_params.push(name.to_string());
            }
        }
    }
    if required_params.len() != 0 {
        return Err(ParsingError::MissingRequiredParameters(required_params));
    }

    Ok(ParsedArgs {
        name,
        flags,
        params,
        unnameds,
        subcommand,
    })
}
