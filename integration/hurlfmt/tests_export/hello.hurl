GET http://localhost:8000/hello
HTTP 200
Content-Type: text/html; charset=utf-8
Content-Length: 12
[Asserts]
header "Date" exists
`Hello World!`

GET http://localhost:8000/hello
HTTP 200
file, data.txt;

GET http://localhost:8000/hello
HTTP 200
hex, 48656c6c6f20576f726c6421;

GET http://localhost:8000/hello
HTTP 200
base64, SGVsbG8gV29ybGQh;
