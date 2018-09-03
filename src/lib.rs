//! **Clargs is a library for parsing command-line arguments.**
//!
//! Parsing command-line arguments requires quite a bit of boilerplate code.
//! This library remedies that for rust projects.
//!
//! Before parsing an argument list a `ParsingConfig` object must be configured.
//! It controls which features are enabled and how certain edge cases should be handled.
//! It is also used to add flags, parameters, subcommands and specify which parameters are required and other extra info.
//! Once configured, it can be used with the `parse` function.
//!
//! A flag is an option that has a name and does not require an argument.
//! A named parameter is an option that has a name and that does require an argument.
//! An unnamed parameter is an option that is neither a flag, a named parameter, nor an argument to a named parameter.
//! An alias maps one name to an option that already has another name.
//!
//! Options are identified by the name they were registered with.
//! Aliases are only used during interpretation of the command-line arguments.
//! As a result of this, they will never occur in the results or an error after parsing.
//!
//! All option names and aliases can contain only lower and uppercase letters and hyphens.
//!
//! Note that the first argument is always interpreted as the name of the command (or subcommand) which was invoked.

mod parsing_error;
mod parsing_config;
mod parsed_args;
mod parse;

pub use self::parsing_error::*;
pub use self::parsing_config::*;
pub use self::parsed_args::*;
pub use self::parse::*;
