GET http://localhost:8000/error-assert-xpath
HTTP/1.0 200
[Asserts]
xpath "strong(//head/title)" equals "Welcome to Quiz!"
