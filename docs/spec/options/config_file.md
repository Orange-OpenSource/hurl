## Configuration File Format

Options may be specified in a configuration file.

The format is similar to the configuration file format used by
[ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md#configuration-file).


### Parsing Rules

- Options must begin with --.
- Each non-empty line represents a single argument after trimming leading and trailing whitespace.
- Lines that begin with `#` (optionally preceded by whitespace) are treated as comments and ignored.
- Empty lines are ignored.
- No shell parsing (e.g., variable expansion or escaping) is performed.


### Option value

- An option value may appear:
  - either on the same line as the option, separated by `=`
  - or on the following line (can still have empty lines between)
- Values containing spaces or newlines must be enclosed in double quotes (").
- Double-quoted values may span multiple lines; newlines are preserved. A quoted value ends at the next double quote (")
- Values starting with `-` are allowed. When using the “next line” form, the next line is always interpreted as a value, not as an option.


### Empty Values

The following forms are equivalent and represent an empty value:

--option=
--option=""
--option
""

### Errors

The following conditions must result in an error:

- Unknown option
- An option that requires a value is not provided one
- Unclosed double quotes
- Unquoted values containing spaces or newlines




### Example

```bash
$ cat $HOME/.config/hurl/config

# Standalone flag
--test

# Provide value inline
--header=foo:bar

# Provide value in the next line
--variable
user=bob

# Use unnecessary quotes
--retry="2"

# Use space in value
--user-agent="Mozilla/5.0 A"

# Use multiple line value
--variable "lines=line1
line2
line3"

# Use empty value
--user-agent=

# Use value that starts with - using "the next-line" form
--retry
-1

# This is recommended (easier to read) to use the same line
# for option staring with `-`
--retry=-1

```

