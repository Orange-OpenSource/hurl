GET http://localhost:8000/hello
HTTP 200
[Asserts]
jsonpath "$.toto" == 23
[Asserts]
jsonpath "$.tata" == 24

