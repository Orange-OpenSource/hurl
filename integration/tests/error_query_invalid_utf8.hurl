GET http://localhost:8000/error-query-invalid-utf8
HTTP/1.0 200
[Asserts]
jsonpath "$.errors" countEquals 2

