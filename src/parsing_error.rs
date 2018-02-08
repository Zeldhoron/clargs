/// Returned when an error occurs during parsing.
///
/// Indicates the type of error that occurred and contains some extra data about the error.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParsingError {

    /// Indicates an unknown option was passed to the parser.
    /// The associated string is set to the affected option's name.
    Unrecognized(String),

    /// Indicates an argument could be multiple valid options.
    /// The associated string is set to the affected argument.
    /// The associated vector of strings is set to the valid options that the argument could have been.
    Ambiguous(String, Vec<String>),

    /// Indicates a named parameter is missing an argument.
    /// The associated string is set to the affected parameter's name.
    MissingArgument(String),

    /// Indicates a named parameter's argument could not be parsed into the parameter's associated type.
    /// The first associated string is set to the affected parameter's name.
    /// The second associated string is set to the affected parameter's argument.
    ArgumentParsingError(String, String),

    /// Indicates a named parameter was set multiple times.
    /// The associated string is set to the affected parameter's name.
    ParameterRepetition(String),
}

impl ParsingError {

    /// Returns a default error message for the specified `ParsingError`.
    pub fn default_message(&self) -> String {
        match *self {
            ParsingError::Unrecognized(ref option) => format!("error: unrecognized option \'{}\'", option),
            ParsingError::Ambiguous(ref arg, ref options) => format!("error: argument \'{}\' is ambiguous for:\n{:?}", arg, options),
            ParsingError::MissingArgument(ref param) => format!("error: option \'{}\' is missing an argument", param),
            ParsingError::ArgumentParsingError(ref param, ref arg) => format!("error: argument \'{}\' is not a valid value for option \'{}\'", arg, param),
            ParsingError::ParameterRepetition(ref param) => format!("error: option \'{}\' can be specified only once", param),
        }
    }
}
