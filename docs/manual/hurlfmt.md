## NAME

hurlfmt - format Hurl files


## SYNOPSIS

**hurlfmt** [options] [FILE...]


## DESCRIPTION

**hurlfmt** formats Hurl files and converts them from/to other formats.

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


hurlfmt can also be used to convert a curl command-line to Hurl

```
$ echo "curl http://localhost:8000/custom-headers -H 'Fruit:Raspberry'" | hurlfmt --in curl
GET http://localhost:8000/custom-headers
Fruit: Raspberry
```


## OPTIONS


### --check {#check}

Run in check mode. Exits with 0 if input is formatted correctly, 1 otherwise.

This can not be used with [--output](#output).

This option is not stable yet.

### --color {#color}

Colorize Output.

This can not be used [--in-place](#inplace).

### --in <FORMAT> {#in}

Specify input format: hurl or curl.

### --in-place {#in-place}

Modify file in place.

This can be used only with text output.

### --no-color {#no-color}

Do not colorize output.

### --out <FORMAT> {#out}

Specify output format: hurl, json or html.

### -o, --output <FILE> {#output}

Write output to FILE instead of stdout.

### --standalone {#standalone}

Output full html file with css instead of html fragment (default).

This can be used only with html output.

### -h, --help {#help}

Usage help.


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
