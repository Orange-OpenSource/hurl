GET http://localhost:8000/hello
HTTP 200
[Asserts]
header "toto" tata startsWith "hello"
