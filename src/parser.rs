use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;

use param_parser::ParamParserTyped;
use param_parser::ParamParser;
use parsing_results::ParsingResults;
use parsing_error::ParsingError;


/// Object used to parse command-line arguments.
///
/// Holds information required to parse arguments and has methods to access this information.
#[derive(Default)]
pub struct Parser {
    flags: HashSet<String>,
    named_params: HashMap<String, Box<ParamParser>>,
    aliases: HashMap<String, String>,
    double_hyphen_marker: bool,
}


impl Parser {
    /// Constructs and returns a new `Parser` object.
    pub fn new() -> Parser {
        Parser {
            flags: HashSet::<String>::new(),
            named_params: HashMap::<String, Box<ParamParser>>::new(),
            aliases: HashMap::<String, String>::new(),
            double_hyphen_marker: true,
        }
    }

    /// Returns `true` if the specified `name` is a valid option name.
    /// An option's name can only contain lower and uppercase letters and hyphens.
    pub fn is_valid_option_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        for chr in name.chars() {
            if !chr.is_alphabetic() && chr != '-' {
                return false;
            }
        }
        true
    }


    /// Returns `true` if the `Parser` currently has an option or an alias with the specified `name`.
    pub fn has_option(&self, name: &str) -> bool {
        self.flags.contains(name) || self.named_params.contains_key(name) || self.aliases.contains_key(name)
    }

    /// Invalidates any option with the specified `name` from the `Parser` object.
    /// Does nothing if there is no such option.
    ///
    /// If an option is removed, all aliases to that option will be removed aswell.
    /// If an alias is removed, only that alias will be removed and nothing else.
    pub fn remove_option(&mut self, name: &str) {
        self.aliases.remove(name);
        self.flags.remove(name);
        self.named_params.remove(name);
        for alias in self.aliases.iter().find(|&(_, option)| option == name).map(|(alias, _)| alias.clone()) {
            self.aliases.remove(&alias);
        }
    }


    /// Registers a new flag with the `Parser` under the specified `name`.
    ///
    /// # Panics
    ///
    /// Panics if there already is an option or an alias with the specified `name`.
    /// Or if the specified `name` is an invalid option name.
    pub fn add_flag(&mut self, name: String) -> &mut Parser {
        if self.has_option(&name) {
            panic!("clargs: cannot add an option with a name that is already present!");
        }
        if !Parser::is_valid_option_name(&name) {
            panic!("clargs: cannot add an option with an invalid name!");
        }
        self.flags.insert(name);
        self
    }

    /// Registers a new named parameter with the `Parser` under the specified `name`.
    /// The named parameter's associated type is set to the `T` type.
    ///
    /// # Panics
    ///
    /// Panics if there already is an option or an alias with the specified `name`.
    /// Or if the specified `name` is an invalid option name.
    pub fn add_named_param<T: 'static + FromStr>(&mut self, name: String) -> &mut Parser {
        if self.has_option(&name) {
            panic!("clargs: cannot add an option with a name that is already present!");
        }
        if !Parser::is_valid_option_name(&name) {
            panic!("clargs: cannot add an option with an invalid name!");
        }
        self.named_params.insert(name, Box::new(ParamParserTyped::<T>::new()));
        self
    }

    /// Registers a new alias with the `Parser` with the specified `name` to the specified `target`.
    ///
    /// # Panics
    ///
    /// Panics if there already is an option or an alias with the specified `name`.
    /// Or if the specified `name` is an invalid option name.
    /// Or if the specified `target` is not a present option.
    pub fn add_alias(&mut self, name: String, target: String) -> &mut Parser {
        if self.has_option(&name) {
            panic!("clargs: cannot add an alias with a name that is already present!");
        }
        if !self.flags.contains(&target) && !self.named_params.contains_key(&target) {
            panic!("clargs: cannot add an alias with a target that does not exist!");
        }
        if !Parser::is_valid_option_name(&name) {
            panic!("clargs: cannot add an alias with an invalid name!");
        }
        self.aliases.insert(name, target);
        self
    }
    

    /// Returns `true` if the double-hyphen marker is enabled.
    pub fn double_hyphen_marker(&self) -> bool {
        self.double_hyphen_marker
    }

    /// Enables the double-hyphen marker.
    pub fn enable_double_hyphen_marker(&mut self) {
        self.double_hyphen_marker = true;
    }

    /// Disables the double-hyphen marker.
    pub fn disable_double_hyphen_marker(&mut self) {
        self.double_hyphen_marker = false;
    }


    /// Matches the specified `partial` option name to the options registered with the `Parser`.
    /// A `partial` matches an option if that option's name starts with `partial`.
    /// Matches are returned as a sorted vector of strings containing the options's names.
    ///
    /// If an exact match occurs, only that option will be returned and all other matches are discarded.
    pub fn match_options(&self, partial: &str) -> Vec<String> {
        let mut matches = Vec::<String>::new();
        
        for flag in &self.flags {
            if flag == partial {
                matches.clear();
                matches.push(flag.clone());
                return matches;
            } else if flag.starts_with(partial) {
                matches.push(flag.clone());
            }
        }

        for param in self.named_params.keys() {
            if param == partial {
                matches.clear();
                matches.push(param.clone());
                return matches;
            } else if param.starts_with(partial) {
                matches.push(param.clone());
            }
        }

        matches.sort();
        matches
    }

    /// Returns the definite name of an option.
    /// If the specified `name` is an alias, returns the name of the option it points to.
    /// Otherwise, returns the same string.
    pub fn resolve(&self, name: String) -> String {
        match self.aliases.get(&name) {
            Some(value) => value.clone(),
            None => name,
        }
    }


    /// Parses the specified command-line arguments.
    /// If an error occurred according to the `Parser`'s configuration, a `ParsingError` will be returned.
    /// Otherwise, a `ParsingResults` object will be returned containing the results.
    pub fn parse<I>(&self, mut args: I) -> Result<ParsingResults, ParsingError>
    where
        I: Iterator,
        I::Item: AsRef<str>,
    {
        let mut results = ParsingResults::new();

        if let Some(arg) = args.next() {
            *results.path_mut() = String::from(arg.as_ref());
        }

        while let Some(arg) = args.next() {
            let arg = arg.as_ref();

            if self.double_hyphen_marker && arg == "--" {
                while let Some(arg) = args.next() {
                    results.unnamed_params_mut().push(String::from(arg.as_ref()));
                }
            } else if arg.starts_with("--") && arg.chars().count() > 2 {
                if arg.contains('=') {
                    let sep = arg.find('=').unwrap();
                    let option = &arg[2..sep];
                    let value = &arg[sep+1..];

                    let mut matches = self.match_options(option);
                    let option = match matches.len() {
                        0 => return Err(ParsingError::Unrecognized(String::from(&arg[2..]))),
                        1 => self.resolve(matches.remove(0)),
                        _ => return Err(ParsingError::Ambiguous(String::from(option), matches)),
                    };

                    if let Some(param_parser) = self.named_params.get(&option) {
                        if results.named_params().contains_key(&option) {
                            return Err(ParsingError::ParameterRepetition(option));
                        }

                        match param_parser.parse(value) {
                            Some(value_box) => results.named_params_mut().insert(option, value_box),
                            None => return Err(ParsingError::ArgumentParsingError(option, String::from(value))),
                        };
                    } else {
                        return Err(ParsingError::Unrecognized(String::from(&arg[2..])));
                    }
                } else {
                    let option = &arg[2..];

                    let mut matches = self.match_options(option);
                    let option = match matches.len() {
                        0 => return Err(ParsingError::Unrecognized(String::from(option))),
                        1 => self.resolve(matches.remove(0)),
                        _ => return Err(ParsingError::Ambiguous(String::from(option), matches)),
                    };

                    if self.flags.contains(&option) {
                        results.flags_mut().insert(option);
                    } else if let Some(param_parser) = self.named_params.get(&option) {
                        if results.named_params().contains_key(&option) {
                            return Err(ParsingError::ParameterRepetition(option));
                        }

                        let value = match args.next() {
                            Some(value) => value,
                            None => return Err(ParsingError::MissingArgument(option)),
                        };

                        match param_parser.parse(value.as_ref()) {
                            Some(value_box) => results.named_params_mut().insert(option, value_box),
                            None => return Err(ParsingError::ArgumentParsingError(option, String::from(value.as_ref()))),
                        };
                    } else {
                        return Err(ParsingError::Unrecognized(String::from(&arg[2..])));
                    }
                }
            } else if arg.starts_with('-') && arg.chars().count() > 1 {
                for (i, chr) in arg.chars().skip(1).enumerate() {
                    let option = self.resolve(chr.to_string());

                    if self.flags.contains(&option) {
                        results.flags_mut().insert(option);
                    } else if let Some(param_parser) = self.named_params.get(&option) {
                        if results.named_params().contains_key(&option) {
                            return Err(ParsingError::ParameterRepetition(option));
                        }
                        
                        if i == arg.chars().count() - 2 {
                            let value = match args.next() {
                                Some(value) => value,
                                None => return Err(ParsingError::MissingArgument(option)),
                            };

                            match param_parser.parse(value.as_ref()) {
                                Some(value_box) => results.named_params_mut().insert(option, value_box),
                                None => return Err(ParsingError::ArgumentParsingError(option, String::from(value.as_ref()))),
                            };
                        } else if arg.chars().nth(i + 2).unwrap().is_digit(10) {
                            let mut value = String::new();
                            for chr in arg.chars().skip(i + 2) {
                                value.push(chr);
                            }

                            match param_parser.parse(value.as_ref()) {
                                Some(value_box) => results.named_params_mut().insert(option, value_box),
                                None => return Err(ParsingError::ArgumentParsingError(option, String::from(value.as_ref()))),
                            };

                            break;
                        } else {
                            return Err(ParsingError::MissingArgument(option));
                        }
                    } else {
                        return Err(ParsingError::Unrecognized(option));
                    }
                }
            } else {
                results.unnamed_params_mut().push(String::from(arg));
            }
        }

        Ok(results)
    }
}
