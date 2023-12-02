# Testing invalid jsonpath expression
GET http://localhost:8000/error-invalid-jsonpath
HTTP 200
[Asserts]
jsonpath "" == false
jsonpath "$.tags[0]x" == false
jsonpath "$.tags[0,A]" == false
jsonpath "$.tags[0:A]" == false
jsonpath "$.tags[]" == false
jsonpath "$." == false
