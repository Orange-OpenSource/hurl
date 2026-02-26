## Configuration File Format

Options may be specified in a configuration file.

The format is similar to the configuration file format used by
[ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md#configuration-file).

### Parsing Rules

- Each non-empty line represents a single shell argument, after trimming leading and trailing whitespace.
- Lines that begin with `#` (optionally preceded by whitespace) are treated as comments and ignored.
- Empty lines are ignored.
- An option value may appear:
  - either on the same line as the option, separated by `=`
  - or on the following line

### Example

```bash
$ cat $HOME/.config/hurl/config

# Execute in test mode
--test

# Pass custom header to each request
--header=foo:bar

# Set variable user
--variable
user=bob

In this example:

--test is a standalone flag.

--header=foo:bar provides its value inline.

--variable takes its value from the next line (user=bob).
