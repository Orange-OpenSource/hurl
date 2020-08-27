GET http://localhost:8000/error-assert-value
HTTP/1.0 200
[Asserts]
header "content-type" equals "XXX"
jsonpath "$.id" equals "000001"
jsonpath "$.values" includes 100

