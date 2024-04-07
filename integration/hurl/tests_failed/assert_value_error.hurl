GET http://localhost:8000/error-assert-value
HTTP 200
[Asserts]
header "content-type" == "XXX"
header "content-type" != "text/html; charset=utf-8"
jsonpath "$.id" == "000001"
jsonpath "$.values" includes 100
jsonpath "$.values" not contains "Hello"
jsonpath "$.count" > 5
jsonpath "$.count" isFloat
jsonpath "$.line_terminator" == "\r\n"
bytes contains hex,00;
