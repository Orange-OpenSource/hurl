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
# invalid regex expression
jsonpath "$.items[?match(@.name, '[]')].name" not exists
jsonpath "$.items[?search(@.name, '[]')]" not exists
# invalid type for regex
jsonpath "$.items[?match(@.name, 1)].name" not exists
jsonpath "$.items[?search(@.name, 1)]" not exists
