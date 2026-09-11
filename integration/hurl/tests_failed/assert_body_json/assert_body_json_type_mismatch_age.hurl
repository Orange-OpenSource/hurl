# Integer value mismatch
GET http://localhost:8000/assert-body-json/type_mismatch_age
HTTP 200
[Asserts]
jsonpath "$.age" isNumber
jsonpath "$.age" == 22
