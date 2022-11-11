GET http://localhost:8000/unused
HTTP 200
[Asserts]
jsonpath "$.data" == /aa{a}/
