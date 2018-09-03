use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error as FmtError;


/// Returned when an error occurs during the parsing of an argument list.
#[derive(PartialEq, Eq)]
pub enum ParsingError {
    /// Indicates that an option was specified which the program does not recognize.
    ///
    /// The associated string is the name of the aforementioned option.
    UnrecognizedOption(String),

    /// Indicates that a specified option could mean multiple valid options.
    ///
    /// The associated string is the name of the aformentioned option.
    /// The associated vector of strings contains all the possible valid options.
    AmbiguousOption(String, Vec<String>),

    /// Indicates that a value was assigned to an option which does not take a value.
    ///
    /// The associated string is the name of the aforementioned option.
    AssignmentToFlag(String),

    /// Indicates that a value was assigned to an option which does not take a value.
    ///
    /// The first associated string is the name of the aforementioned option.
    /// The second associated string is the name of the alias which was used to specify the option.
    AssignmentToFlagAlias(String, String),

    /// Indicates that a value was assigned to the same option more than once.
    ///
    /// The associated string is the name of the aforementioned option.
    ParameterDuplication(String),

    /// Indicates that a value was assigned to the same option more than once.
    ///
    /// The first associated string is the name of the aforementioned option.
    /// The second associated string is the name of the alias which was used to specify the option.
    ParameterDuplicationAlias(String, String),

    /// Indicates that an option was specified which takes a value but didn't get one.
    ///
    /// The associated string is the name of the aforementioned option.
    MissingArgument(String),

    /// Indicates that an option was specified which takes a value but didn't get one.
    ///
    /// The first associated string is the name of the aforementioned option.
    /// The second associated string is the name of the alias which was used to specify the option.
    MissingArgumentAlias(String, String),

    /// Indicates that a subcommand was specified which the program does not recognize.
    ///
    /// The associated string is the name of the aforementioned subcommand.
    UnrecognizedSubcommand(String),

    /// Indicates that a specified subcommand could mean multiple valid subcommands.
    ///
    /// The associated string is the name of the aformentioned subcommand.
    /// The associated vector of strings contains all the possible valid subcommands.
    AmbiguousSubcommand(String, Vec<String>),

    /// Indicates that a required subcommand is missing from the argument list.
    MissingRequiredSubcommand,

    /// Indicates that one or more required options are missing from the argument list.
    ///
    /// The associated vector of strings contains all the missing required options.
    MissingRequiredParameters(Vec<String>),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            ParsingError::UnrecognizedOption(name) => write!(f, "unrecognized option '{}'", name),
            ParsingError::AmbiguousOption(name, matches) => {
                write!(f, "option '{}' is ambiguous; possibilities:", name)?;
                for value in matches {
                    write!(f, " '{}'", value)?;
                }
                Ok(())
            },
            ParsingError::AssignmentToFlag(name) => write!(f, "option '{}' doesn't allow an argument", name),
            ParsingError::AssignmentToFlagAlias(name, alias) => write!(f, "option '{}' doesn't allow an argument; note that '{}' is an alias to '{0}'", name, alias),
            ParsingError::ParameterDuplication(name) => write!(f, "parameter '{}' was set more than once", name),
            ParsingError::ParameterDuplicationAlias(name, alias) => write!(f, "parameter '{}' was set more than once; note that '{}' is an alias to '{0}'", name, alias),
            ParsingError::MissingArgument(name) => write!(f, "parameter '{}' is missing an argument", name),
            ParsingError::MissingArgumentAlias(name, alias) => write!(f, "parameter '{}' is missing an argument; note that '{}' is an alias to '{0}'", name, alias),
            ParsingError::UnrecognizedSubcommand(name) => write!(f, "unrecognized subcommand '{}'", name),
            ParsingError::AmbiguousSubcommand(name, matches) => {
                write!(f, "subcommand '{}' is ambiguous; possibilities:", name)?;
                for value in matches {
                    write!(f, " '{}'", value)?;
                }
                Ok(())
            },
            ParsingError::MissingRequiredSubcommand => write!(f, "argument list is missing a required subcommand"),
            ParsingError::MissingRequiredParameters(names) => match names.len() {
                0 => write!(f, "argument list is missing a required parameter"),
                1 => write!(f, "argument list is missing required parameter '{}'", names[0]),
                _ => {
                    write!(f, "argument list is missing required parameters:")?;
                    for value in names {
                        write!(f, " '{}'", value)?;
                    }
                    Ok(())
                },
            },
        }
    }
}

impl Debug for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        (self as &Display).fmt(f)
    }
}

impl Error for ParsingError {
    fn cause(&self) -> Option<&Error> {
        None
    }
}
