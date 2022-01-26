GET http://localhost:8000/unused
HTTP/1.1 200
[Asserts]
jsonpath "$.data" == /aa{a}/
