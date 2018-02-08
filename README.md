The `clargs` library can be used to parse command-line arguments.

Parsing command-line arguments requires quite a bit of boilerplate code.
This library remedies that for rust projects.

A `Parser` object can be used to parse command-line arguments after configuring the `Parser`.
Configuring the `Parser` consists of registering flags and named parameters, and (optionally) enabling or disabling some features.

A flag is an option that has a name and does not require an argument.
A named parameter is an option that has a name and that does require an argument.
An unnamed parameter is an option that is neither a flag, a named parameter, nor an argument to a named parameter.

All option names can contain only lower and uppercase letters and hyphens.

The double hyphen marker is a feature that is enabled by default.
If enabled, any arguments that follow a '--' argument are interpreted as unnamed parameters.
If disabled, the '--' argument itself is interpreted as an unnamed parameter.

# Syntax

There are two types of command-line syntax, single hyphen and double hyphen syntax.

Single hyphen syntax works by specifying options as letters preceded by a '-'.
Any letters following a single '-' are interpreted as options.
If the last option is a named parameter, then the next argument is that option's argument.
If another option is a named parameter, then the next letter must be a digit.
In that case, the digit and all following letters are interpreted as that option's argument.

Double hyphen syntax works by specifying options by preceding them with a '--'.
If the string following a '--' does not match any option's name, a partial match will be looked for.
A string partially matches an option's name, if that name starts with the specified string.
Note that if there are multiple matches, an error will be returned.
If the specified option is a named parameter, the next argument will be interpreted as that option's argument.

If an option specified using '--' contains a '=', it is interpreted as a named parameter.
The part before the '=' is interpreted as the option's name, the part after the '-' is interpreted as the option's argument.

# Examples

An example of how `clargs` would be used:

```
// create and configure a parser object
let mut parser = clargs::Parser::new();

parser.add_flag(String::from("f"));
parser.add_flag(String::from("flag"));

parser.add_named_param::<i32>(String::from("i"));
parser.add_named_param::<i32>(String::from("int"));

parser.add_named_param::<String>(String::from("s"));
parser.add_named_param::<String>(String::from("str"));


// getting and parsing the command line arguments
let args: Vec<String> = std::env::args().collect();
let results = match parser.parse(args.iter()) {
    Ok(results) => results,
    Err(error) => {
        // on an error, print the default error message and exit
        println!("{}", error.default_message());
        return;
    },
};


// code with path
println!("{}", results.path());

if results.flag("f") {
    // code if flag 'f' was set
}
if results.flag("flag") {
    // code if flag 'flag' was set
}

if let Some(value) = results.named_param::<i32>("i") {
    // code if parameter 'i' was set
    // with 'value' equal to the parameter's argument of type 'i32'
}
if let Some(value) = results.named_param::<i32>("int") {
    // code if parameter 'int' was set
    // with 'value' equal to the parameter's argument of type 'i32'
}

if let Some(value) = results.named_param::<String>("s") {
    // code if parameter 's' was set
    // with 'value' equal to the parameter's argument of type 'String'
}
if let Some(value) = results.named_param::<String>("str") {
    // code if parameter 'str' was set
    // with 'value' equal to the parameter's argument of type 'String'
}

// code with unnamed parameters
println!("{:?}", results.unnamed_params());
```
