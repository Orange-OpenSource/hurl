GET http://localhost:8000/subquery-count
HTTP/1.0 200
[Asserts]
jsonpath "$.users" count == 3
jsonpath "$.users" count > 1
jsonpath "$.users" count <= 10

