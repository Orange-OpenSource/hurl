GET http://localhost:8000/error-query-invalid-json
HTTP/1.0 200
[Asserts]
jsonpath "$.errors" count == 2

