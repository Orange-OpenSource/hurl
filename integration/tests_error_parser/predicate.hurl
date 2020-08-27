GET http://localhost:8000/hello
[Asserts]
header "toto" tata startsWith "hello"
