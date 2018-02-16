use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;
use std::any::Any;

/// Returned after successfully parsing command-line arguments.
///
/// Contains all the parsed results of the arguments with methods to retrieve those results.
#[derive(Default, Debug)]
pub struct ParsingResults {
    path: String,
    flags: HashSet<String>,
    named_params: HashMap<String, Box<Any>>,
    unnamed_params: Vec<String>,
}

impl ParsingResults {
    /// Constructs and returns a new `ParsingResults` object.
    pub fn new() -> ParsingResults {
        ParsingResults {
            path: String::new(),
            flags: HashSet::<String>::new(),
            named_params: HashMap::<String, Box<Any>>::new(),
            unnamed_params: Vec::<String>::new(),
        }
    }


    /// Returns a reference to the stored path.
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Returns a mutable reference to the stored path.
    pub fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    /// Returns a reference to the stored set of flags.
    pub fn flags(&self) -> &HashSet<String> {
        &self.flags
    }

    /// Returns a mutable reference to the stored set of flags.
    pub fn flags_mut(&mut self) -> &mut HashSet<String> {
        &mut self.flags
    }

    /// Returns a reference to the stored map of named parameters to their respective values.
    pub fn named_params(&self) -> &HashMap<String, Box<Any>> {
        &self.named_params
    }

    /// Returns a mutable reference to the stored map of named parameters to their respective values.
    pub fn named_params_mut(&mut self) -> &mut HashMap<String, Box<Any>> {
        &mut self.named_params
    }

    /// Returns a reference to the stored vector of unnamed parameters.
    pub fn unnamed_params(&self) -> &Vec<String> {
        &self.unnamed_params
    }

    /// Returns a mutable reference to the stored vector of unnamed parameters.
    pub fn unnamed_params_mut(&mut self) -> &mut Vec<String> {
        &mut self.unnamed_params
    }


    /// Returns `true` if a flag with the specified `name` was set in the parsed arguments.
    pub fn flag(&self, name: &str) -> bool {
        self.flags.contains(name)
    }

    /// Returns a reference to the specified named parameter's value.
    /// If no such parameter was set in the parsed arguments, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the specified type does not match the specified parameter's associated type.
    pub fn named_param<T: 'static + FromStr>(&self, name: &str) -> Option<&T> {
        match self.named_params.get(name) {
            Some(value) => {
                if !value.is::<T>() {
                    panic!("clargs: specified type does not match the specified parameter's associated type");
                }
                Some(value.downcast_ref::<T>().unwrap())
            },
            None => None,
        }
    }

    /// Returns a mutable reference to the specified named parameter's value.
    /// If no such parameter was set in the parsed arguments, returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the specified type does not match the specified parameter's associated type.
    pub fn named_param_mut<T: 'static + FromStr>(&mut self, name: &str) -> Option<&mut T> {
        match self.named_params.get_mut(name) {
            Some(value) => {
                if !value.is::<T>() {
                    panic!("clargs: specified type does not match the specified parameter's associated type");
                }
                Some(value.downcast_mut::<T>().unwrap())
            },
            None => None,
        }
    }

    /// Returns the number of stored unnamed parameters.
    pub fn num_unnamed_params(&self) -> usize {
        self.unnamed_params.len()
    }

    /// Returns a reference to the unnamed parameter at the specified zero-based `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is higher than or equal to the number of stored unnamed parameters.
    pub fn unnamed_param(&self, index: usize) -> &String {
        if index >= self.unnamed_params.len() {
            panic!("clargs: attempted to access a non-existant unnamed parameter");
        }
        &self.unnamed_params[index]
    }

    /// Returns a mutable reference to the unnamed parameter at the specified zero-based `index`.
    ///
    /// # Panics
    ///
    /// Panics if the `index` is higher than or equal to the number of stored unnamed parameters.
    pub fn unnamed_param_mut(&mut self, index: usize) -> &mut String {
        if index >= self.unnamed_params.len() {
            panic!("clargs: attempted to access a non-existant unnamed parameter");
        }
        &mut self.unnamed_params[index]
    }
}
