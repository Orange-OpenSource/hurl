GET http://localhost:8000/assert-json
HTTP/1.0 200
[Asserts]
jsonpath "$.success" equals false
jsonpath "$.success" not equals null
jsonpath "$.success" exists
jsonpath "$.errors" countEquals 2
jsonpath "$.warnings" countEquals 0
jsonpath "$.toto" not exists
jsonpath "$.warnings" exists
jsonpath "$.warnings" exists
jsonpath "$.errors[0]" exists
jsonpath "$.errors[0].id" equals "error1"
jsonpath "$.errors[0]['id']" equals "error1"
jsonpath "$.duration" equals 1.5
jsonpath "$.duration" lessThanOrEquals 2.0
jsonpath "$.duration" lessThan 2
jsonpath "$.nullable" equals null

{
  "success": false,
  "errors": [{"id":"error1"},{"id":"error2"}],
  "warnings": [],
  "duration": 1.5,
  "tags": ["test"],
  "nullable": null
}

GET http://localhost:8000/assert-json/index
HTTP/1.0 200
[Captures]
index: body

GET http://localhost:8000/assert-json
HTTP/1.0 200
[Asserts]
jsonpath "$.errors[{{index}}].id" equals "error2"
jsonpath "$.tags" includes "test"
jsonpath "$.tags" not includes "prod"
jsonpath "$.tags" not includes null


GET http://localhost:8000/assert-json/list
HTTP/1.0 200
[Asserts]
jsonpath "$" countEquals 2
jsonpath "$.[0].name" equals "Bob"
jsonpath "$[0].name" equals "Bob"
