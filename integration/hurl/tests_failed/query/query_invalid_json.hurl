GET http://localhost:8000/error-query-invalid-json
HTTP 200
[Asserts]
jsonpath "$.errors" count == 2

