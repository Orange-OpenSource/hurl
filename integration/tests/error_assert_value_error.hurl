GET http://localhost:8000/error-assert-value
HTTP/1.0 200
[Asserts]
header "content-type" equals "XXX"
header "content-type" notEquals "text/html; charset=utf-8"
jsonpath "$.id" equals "000001"
jsonpath "$.values" includes 100
jsonpath "$.values" not contains "Hello"
jsonpath "$.count" greaterThan 5
jsonpath "$.count" isFloat
bytes contains hex,00;