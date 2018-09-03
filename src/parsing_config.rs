use std::collections::HashMap;
use std::collections::HashSet;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ArgDesc {
    Flag,
    Param(bool),
    Alias(String),
}


/// Controls how the argument list is interpreted.
///
/// A `ParsingConfig` struct must be configured before parsing an argument list.
/// It controls which syntax is allowed, how certain edge cases should be handled and the desired flags, parameters and subcommands.
///
/// # Syntax support
///
/// Clargs supports a number of common command-line argument syntaxes and options to change how certain edge cases are handled.
/// To enable or disable syntaxes or features a `ParsingConfig` object must be constructed and modified accordingly.
///
/// A list of all supported syntaxes and features:
/// - parameters, flags and unnamed parameters
/// - double hyphen assignment syntax
/// - double hyphen syntax
/// - single hyphen syntax
/// - assignment syntax
/// - subcommands
/// - option completion
/// - subcommand completion
///
/// ### Parameters, flags and unnamed parameters
///
/// There are three types of options: parameters, flags and unnamed parameters.
/// Parameters are optionally required options with a name which take a value.
/// Flags are options with a name which do not take a value.
/// Unnamed parameters are options without a name, they are the arguments that are not interpreted by any of the syntaxes or features.
///
/// ### Double hyphen assignment syntax
///
/// Double hyphen assignment syntax can only specify parameter options.
///
/// It is used as follows:
///
/// > --NAME=VALUE
///
/// Where "NAME" and "VALUE" can both be replaced by any appropriate value.
///
/// If option completion is enabled, "NAME" can also be any string that is a prefix of the intended option's name.
/// Note that in that case, it is an error if "NAME" matches prefixes of multiple names.
///
/// ### Double hyphen syntax
///
/// Double hyphen syntax can specify both flags and parameters.
///
/// It is used as follows for flags:
///
/// > --NAME
///
/// It is used as follows for parameters:
///
/// > --NAME VALUE
///
/// Where "NAME" and "VALUE" can both be replaced by any appropriate value.
/// In the latter case, the next argument ("VALUE") will always be used to set the parameter's value, regardless of what it might be.
///
/// If option completion is enabled, "NAME" can also be any string that is a prefix of the intended option's name.
/// Note that in that case, it is an error if "NAME" matches prefixes of multiple names.
///
/// ### Single hyphen syntax
///
/// Single hyphen syntax can specify both flags and parameters.
/// But only those that have a name of one character.
///
/// It is used as follows:
///
/// > -FLAGSP VALUE
///
/// Or as follows:
///
/// > -FLAGSP495
///
/// Where each character in the string "FLAGS" must be the name of a flag option.
/// And where "P" is the name of a parameter option and "VALUE" is its value.
/// In the first case, the next argument ("VALUE") will always be used to set the parameter's value, regardless of what it might be.
/// In the latter case, the parameter's value will be set to "495".
/// Once a non-alphabetic character is found in the argument, that character and all following characters are interpreted as the value to the last option that was specified.
///
/// Note that as much flags as desired can be specified in one argument, but at most one parameter.
/// And if there is one, the parameter must always be the last option to be specified.
///
/// ##### Naked hyphen
///
/// The naked hyphen is a special case in the single hyphen syntax.
/// By default it is interpreted as an unnamed parameter.
/// If however, an empty string is added as a flag or parameter to the `ParsingConfig` object, the resulting option can be specified through use of the naked hyphen.
///
/// If it is a flag it can be specified as follows:
///
/// > \-
///
/// If it is a parameter it can be given a value as follows:
///
/// > -495
///
/// Where "495" can be any string not starting with an alphabetic character.
///
/// ##### Parameter stacking
///
/// If enabled, parameter stacking allows for the specification of multiple parameters in a single argument.
///
/// It is used as follows:
///
/// > -FPQLARG VALUE_P VALUE_Q VALUE_R
///
/// Or as follows:
///
/// > -FPQLAR495 VALUE_P VALUE_Q
///
/// Where 'F', 'L', 'A' and 'G' represent flag options and 'P', 'Q' and 'R' parameter options.
/// In the first case, parameter 'P' will get value "VALUE_P", parameter 'Q' will get value "VALUE_Q" and parameter 'R' will get value "VALUE_R".
/// In the latter case, parameter 'P' will get value "VALUE_P", parameter 'Q' will get value "VALUE_Q" and parameter 'R' will get value "495".
/// Once a non-alphabetic character is found in the argument, that character and all following characters are interpreted as the value to the last option that was specified.
/// The next arguments ("VALUE_*") will always be used to set the parameters's values, regardless of what they might be.
///
/// ### Assignment syntax
///
/// Assignment syntax can only specify parameter options.
///
/// It is used as follows:
///
/// > NAME=VALUE
///
/// Where "NAME" and "VALUE" can both be replaced by any appropriate value.
///
/// If option completion is enabled, "NAME" can also be any string that is a prefix of the intended option's name.
/// Note that in that case, it is an error if "NAME" matches prefixes of multiple names.
///
/// ### Subcommands
///
/// Subcommands can be enabled by adding them to the `ParsingConfig` object.
/// If enabled, any unnamed parameters that match the name of a subcommand will be interpreted as such.
/// Once a subcommand is found, all remaining arguments are passed untouched to the subcommand.
///
/// If the subcommand index feature is enabled, the argument parser will look only at the specified index in the unnamed parameters list for a subcommand.
/// The index the argument parser will use can be specified in the `ParsingConfig` object.
/// If the subcommand completion feature is enabled as well, any string that is a prefix of a subcommand will work as a subcommand.
/// Note that in that case, it is an error if the argument matches prefixes of multiple subcommands.
///
/// It is also possible to set whether a subcommand is required or not.
/// If it is required, and the subcommand index feature is enabled as well, it is an error if no subcommand is found at the specified index in the unnamed parameter list.
pub struct ParsingConfig {
    pub(crate) dh_marker: bool,
    pub(crate) store_dh_marker: bool,
    pub(crate) sh_syntax: bool,
    pub(crate) dh_syntax: bool,
    pub(crate) dha_syntax: bool,
    pub(crate) a_syntax: bool,

    pub(crate) param_stacking: bool,
    pub(crate) param_duplication: bool,

    pub(crate) option_completion: bool,
    pub(crate) subcmd_completion: bool,

    pub(crate) subcmd_index: bool,
    pub(crate) subcmd_required: bool,
    pub(crate) subcmd_index_value: usize,

    pub(crate) options: HashMap<String, ArgDesc>,
    pub(crate) subcmds: HashSet<String>,
}

impl ParsingConfig {
    /// Constructs and returns a `ParsingConfig` object.
    pub fn new() -> Self {
        Self {
            dh_marker: true,
            store_dh_marker: false,
            sh_syntax: true,
            dh_syntax: true,
            dha_syntax: true,
            a_syntax: false,

            param_stacking: true,
            param_duplication: false,

            option_completion: true,
            subcmd_completion: false,

            subcmd_index: false,
            subcmd_required: false,
            subcmd_index_value: 0,

            options: HashMap::new(),
            subcmds: HashSet::new(),
        }
    }

    /// Constructs and returns a `ParsingConfig` object with all features disabled.
    pub fn new_all_disabled() -> Self {
        Self {
            dh_marker: false,
            store_dh_marker: false,
            sh_syntax: false,
            dh_syntax: false,
            dha_syntax: false,
            a_syntax: false,

            param_stacking: false,
            param_duplication: false,

            option_completion: false,
            subcmd_completion: false,

            subcmd_index: false,
            subcmd_required: false,
            subcmd_index_value: 0,

            options: HashMap::new(),
            subcmds: HashSet::new(),
        }
    }

    /// Returns `true` if the `name` is a valid name for a flag, parameter, alias or subcommand.
    ///
    /// A name is valid if it doesn't contain whitespace characters, an equals sign, an apostrophe or a quotation mark and if it starts with an alphabetic character or if it is an empty string.
    pub fn is_valid_name(name: &str) -> bool {
        !name.chars().any(|x| x.is_whitespace() || x == '=' || x == '\'' || x == '"') && name.chars().next().unwrap_or('a').is_alphabetic()
    }


    /// Enables or disables the double hyphen marker.
    ///
    /// Enabled by default.
    pub fn set_double_hyphen_marker(&mut self, value: bool) {
        self.dh_marker = value;
    }

    /// Enables or disables the storing of the double hyphen marker.
    ///
    /// Disabled by default.
    pub fn set_store_double_hyphen_marker(&mut self, value: bool) {
        self.store_dh_marker = value;
    }

    /// Enables or disables single hyphen syntax.
    ///
    /// Enabled by default.
    pub fn set_single_hyphen_syntax(&mut self, value: bool) {
        self.sh_syntax = value;
    }

    /// Enables or disables double hyphen syntax.
    ///
    /// Enabled by default.
    pub fn set_double_hyphen_syntax(&mut self, value: bool) {
        self.dh_syntax = value;
    }

    /// Enables or disables double hyphen assignment syntax.
    ///
    /// Enabled by default.
    pub fn set_double_hyphen_assignment_syntax(&mut self, value: bool) {
        self.dha_syntax = value;
    }

    /// Enables or disables assignment syntax.
    ///
    /// Disabled by default.
    pub fn set_assignment_syntax(&mut self, value: bool) {
        self.a_syntax = value;
    }


    /// Enables or disables single hyphen parameter stacking.
    ///
    /// Enabled by default.
    pub fn set_parameter_stacking(&mut self, value: bool) {
        self.param_stacking = value;
    }

    /// Enables or disables parameter duplication.
    /// Setting the value of a parameter more than once will not be seen as an error if enabled.
    ///
    /// Enabled by default.
    pub fn set_parameter_duplication(&mut self, value: bool) {
        self.param_duplication = value;
    }


    /// Enables or disables argument completion.
    ///
    /// Enabled by default.
    pub fn set_option_completion(&mut self, value: bool) {
        self.option_completion = value;
    }

    /// Enables or disables subcommand completion.
    /// Subcommand completion can occur only if the subcommand index feature is enabled.
    ///
    /// Disabled by default.
    pub fn set_subcommand_completion(&mut self, value: bool) {
        self.subcmd_completion = value;
    }


    /// Enables or disables the index at which the subcommand is expected to be found in the unnamed parameters list.
    /// It is a parsing error if this is enabled, a subcommand is required and there is no subcommand found at the specified position.
    ///
    /// Disabled by default.
    pub fn set_subcommand_index(&mut self, value: bool) {
        self.subcmd_index = value;
    }

    /// Enables or disables the requirement of a subcommand.
    ///
    /// Disabled by default.
    pub fn set_subcommand_required(&mut self, value: bool) {
        self.subcmd_required = value;
    }

    /// Sets the index at which the subcommand is expected to be found in the unnamed parameters list.
    ///
    /// The default value is zero.
    pub fn set_subcommand_index_value(&mut self, value: usize) {
        self.subcmd_index_value = value;
    }


    /// Adds a flag to the configuration.
    ///
    /// # Panics
    ///
    /// Panics if the `name` is already taken or if it is an invalid name.
    pub fn add_flag(&mut self, name: String) {
        assert!(Self::is_valid_name(&name), "clargs: invalid flag name");
        assert!(!self.options.contains_key(&name), "clargs: flag name is already taken");
        assert!(!self.subcmds.contains(&name), "clargs: flag name is already taken");
        self.options.insert(name, ArgDesc::Flag);
    }

    /// Adds a parameter to the configuration.
    ///
    /// # Panics
    ///
    /// Panics if the `name` is already taken or if it is an invalid name.
    pub fn add_param(&mut self, name: String, required: bool) {
        assert!(Self::is_valid_name(&name), "clargs: invalid parameter name");
        assert!(!self.options.contains_key(&name), "clargs: parameter name is already taken");
        assert!(!self.subcmds.contains(&name), "clargs: parameter name is already taken");
        self.options.insert(name, ArgDesc::Param(required));
    }

    /// Adds an alias to a flag or parameter to the configuration.
    /// If the `target` is an alias as well, the new alias will point to that alias's `target`.
    ///
    /// # Panics
    ///
    /// Panics if the `name` is equal to the `target` or if the `target` does not point to a flag, parameter or other alias.
    /// Or if the `name` is already taken or if it is an invalid name.
    pub fn add_alias(&mut self, name: String, target: String) {
        assert!(Self::is_valid_name(&name), "clargs: invalid alias name");
        assert!(!self.options.contains_key(&name), "clargs: alias name is already taken");
        assert!(!self.subcmds.contains(&name), "clargs: alias name is already taken");
        assert!(name != target, "clargs: alias target cannot be the same as its name");
        assert!(self.options.contains_key(&target), "clargs: alias must point to a valid target");
        let target = match self.options.get(&target).unwrap() {
            ArgDesc::Alias(target) => target.to_string(),
            _ => target,
        };
        self.options.insert(name, ArgDesc::Alias(target));
    }

    /// Adds a subcommand to the configuration.
    ///
    /// # Panics
    ///
    /// Panics if the `name` is already taken, if it is an invalid name or if it is an empty string.
    pub fn add_subcommand(&mut self, name: String) {
        assert!(!name.is_empty(), "clargs: subcommand name cannot be an empty string");
        assert!(Self::is_valid_name(&name), "clargs: invalid subcommand name");
        assert!(!self.options.contains_key(&name), "clargs: subcommand name is already taken");
        assert!(!self.subcmds.contains(&name), "clargs: subcommand name is already taken");
        self.subcmds.insert(name);
    }
}
