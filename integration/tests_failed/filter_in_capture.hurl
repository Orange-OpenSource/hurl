GET http://localhost:8000/error-filter-in-capture
HTTP 200
[Captures]
id: jsonpath "$.id" toInt
[Asserts]
jsonpath "$.id" toInt == 1
