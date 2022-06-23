# Hurl File

## Character Encoding

Hurl file should be encoded in UTF-8, without byte order mark to the beginning
(while Hurl ignores the presence of a byte order mark
rather than treating it as an error)

## File Extension

Hurl file extension is `.hurl`

## Comments

Comments begin with `#` and continue until the end of line. Hurl file can serve as
a documentation for HTTP based workflows so it can be useful to be very descriptive.

```hurl
# A very simple Hurl file
# with tasty comments...
GET https://www.sample.net
x-app: MY_APP  # Add a dummy header

HTTP/1.1 302   # Check that we have a redirection
[Asserts]
header "Location" exists
header "Location" contains "login"  # Check that we are redirected to the login page
```

## Special Characters in Strings

String can include the following special characters:

- The escaped special characters \" (double quotation mark), \\ (backslash), \b (backspace), \f (form feed),
 \n (line feed), \r (carriage return), and \t (horizontal tab)
- An arbitrary Unicode scalar value, written as \u{n}, where n is a 1–8 digit hexadecimal number

```hurl
GET https://example.org/api

HTTP/1.1 200

# The following assert are equivalent:
[Asserts]
jsonpath "$.slideshow.title" == "A beautiful ✈!"
jsonpath "$.slideshow.title" == "A beautiful \u{2708}!"
```

In some case, (in headers value, etc..), you will also need to escape # to distinguish from a comment.
In the following example:

```hurl
GET https://example.org/api
x-token: BEEF \#STEACK # Some comment
HTTP/1.1 200
```

We're sending a header `x-token` with value `BEEF #STEACK`

