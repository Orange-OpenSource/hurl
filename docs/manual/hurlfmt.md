## NAME

hurlfmt - format Hurl files


## SYNOPSIS

**hurlfmt** [options] [FILE]


## DESCRIPTION

**hurlfmt** formats Hurl files and converts to other formats.

With no FILE, read standard input.


By default, hurlfmt outputs a formatted and colorized version of the input hurl file.

```
$ hurl hello.hurl
GET http://localhost:8000/hello

HTTP/1.0 200
```



hurlfmt can be used to convert to other format.


```
$ hurl hello.hurl --output json | jq
{
  "entries": [
    {
      "request": {
        "method": "GET",
        "url": "http://localhost:8000/hello"
      },
      "response": {
        "version": "HTTP/1.0",
        "status": 200
      }
    }
  ]
}

```



## OPTIONS


### --check {#check}

Run in 'check' mode. Exits with 0 if input is formatted correctly, 1 otherwise. 

This can not be used with [--output](#output).

This option is not stable yet.


### --color {#color}

Colorize Output.
 
This can not be used [--in-place](#inplace).


### --format {#output}

Specify output format: text (default), json or html.


### -h, --help {#help}

Usage help.


### --inplace {#inplace}

Modify file in place.

This can be used only with text output.


### --no-color {#nocolor}

Do not colorize Output.


### --no-format {#noformat}

Do not format output. 

By default text output is automatically formatted.


### -o, --output <file> {#output}

Write output to <file> instead of stdout.


### --standalone {#standalone}

Output full html file with css instead of html fragment (default).
     
This can be used only with html output.


### -V, --version {#version}

Prints version information




## EXIT CODES

### 1

Failed to parse command-line options.


### 2

Input File Parsing Error.


## WWW

[https://hurl.dev](https://hurl.dev)


## SEE ALSO

hurl(1)
