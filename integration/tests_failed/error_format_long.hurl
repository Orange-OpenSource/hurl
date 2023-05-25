GET http://localhost:8000/error-assert-xpath
HTTP 200
Content-Type: text/html
[Asserts]
xpath "string(//head/title)" equals "Welcome!"
xpath "//foo" isEmpty
xpath "//title" count == 2
