GET http://localhost:8000/error-query-invalid-utf8
HTTP 200
[Asserts]
jsonpath "$.errors" count == 2

