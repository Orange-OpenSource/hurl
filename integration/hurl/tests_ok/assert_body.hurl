# Implicit body asserts (base64, multiline, file):

GET http://localhost:8000/assert-body
HTTP 200
```
line1
line2
line3
```

GET http://localhost:8000/assert-body
HTTP 200
file,assert_body.bin;


# Test body response with line ending LF and CRLF.
# We receive the text body "line1\nline2\r\nline3\n"
#
# $ printf "line1\nline2\r\nline3\n" | base64
# bGluZTEKbGluZTINCmxpbmUzCg==

GET http://localhost:8000/assert-body-with-crlf
HTTP 200
base64,bGluZTEKbGluZTINCmxpbmUzCg==;


# Explicit body asserts

GET http://localhost:8000/assert-body
HTTP 200
[Asserts]
body == "line1\nline2\nline3\n"
bytes == file,assert_body.bin;
bytes == hex,6c696e65310a6c696e65320a6c696e65330a;

