GET http://localhost:8000/error-assert-xpath
HTTP 200
[Asserts]
xpath "strong(//head/title)" equals "Welcome to Quiz!"
xpath "strong(//head/title)" isEmpty
