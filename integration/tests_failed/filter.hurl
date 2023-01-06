GET http://localhost:8000/error-filter
HTTP 200
[Asserts]
jsonpath "$.id" toInt == 123
jsonpath "$.status" toInt == 0
jsonpath "$.unknown" toInt == 1
jsonpath "$.list" nth 5 == 3
