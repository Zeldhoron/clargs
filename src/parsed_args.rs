use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;


/// Contains an argument list in parsed format.
pub struct ParsedArgs {
    pub(crate) name: String,
    pub(crate) flags: HashSet<String>,
    pub(crate) params: HashMap<String, String>,
    pub(crate) unnameds: Vec<String>,
    pub(crate) subcommand: Vec<String>,
}

impl ParsedArgs {
    /// Returns a reference to the first argument.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns `true` if the specified flag was in the argument list.
    pub fn has_flag(&self, name: &str) -> bool {
        self.flags.contains(name)
    }

    /// Returns the value that is associated with the specified parameter.
    pub fn get_param(&self, name: &str) -> Option<&str> {
        match self.params.get(name) {
            Some(value) => Some(value.as_str()),
            None => None,
        }
    }

    /// Returns the value that is associated with the specified parameter parsed to the specified type.
    pub fn get_param_as<T: FromStr>(&self, name: &str) -> Option<Result<T, T::Err>> {
        match self.params.get(name) {
            Some(value) => Some(value.parse()),
            None => None,
        }
    }

    /// Returns a reference to the vector of unnamed parameters.
    pub fn unnamed_params(&self) -> &Vec<String> {
        &self.unnameds
    }

    /// Returns a reference to the name of the subcommand that is being invoked.
    pub fn get_subcommand(&self) -> Option<&str> {
        match self.subcommand.get(0) {
            Some(value) => Some(value.as_str()),
            None => None,
        }
    }

    /// Returns a reference to the vector that holds the arguments for the subcommand which is being invoked.
    pub fn subcommand_args(&self) -> &Vec<String> {
        &self.subcommand
    }
}
