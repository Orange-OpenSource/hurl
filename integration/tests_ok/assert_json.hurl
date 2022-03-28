GET http://localhost:8000/assert-json
HTTP/1.0 200
[Asserts]
jsonpath "$.count" == 5
jsonpath "$.count" == 5.0
jsonpath "$.count" equals 5
jsonpath "$.count" != 4
jsonpath "$.count" not equals 4
jsonpath "$.count" > 1
jsonpath "$.count" > 1.0
jsonpath "$.count" greaterThan 1.0
jsonpath "$.success" == false
jsonpath "$.success" != null
jsonpath "$.success" exists
jsonpath "$.success" isBoolean
jsonpath "$.errors" count == 2
jsonpath "$.errors" isCollection
jsonpath "$.warnings" count == 0
jsonpath "$.toto" not exists
jsonpath "$.warnings" exists
jsonpath "$.warnings" exists
jsonpath "$.errors[0]" exists
jsonpath "$.errors[0]" isCollection
jsonpath "$.errors[0].id" == "error1"
jsonpath "$.errors[0]['id']" == "error1"
jsonpath "$.errors[*].id" includes "error1"
jsonpath "$.errors[?(@.id=='error1')].id" == "error1"
jsonpath "$.duration" == 1.5
jsonpath "$.duration" <= 2.0
jsonpath "$.duration" < 2
jsonpath "$.duration" lessThanOrEquals 2.0
jsonpath "$.duration" lessThan 2
jsonpath "$.duration" isFloat
jsonpath "$.duration" not isInteger
jsonpath "$.nullable" == null

{
  "count": 5,
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
jsonpath "$.errors[{{index}}].id" == "error2"
jsonpath "$.tags" includes "test"
jsonpath "$.tags" not includes "prod"
jsonpath "$.tags" not includes null


GET http://localhost:8000/assert-json/list
HTTP/1.0 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$.[0].name" == "Bob"
jsonpath "$[0].name" == "Bob"

GET http://localhost:8000/assert-json/filter
HTTP/1.0 200
[Asserts]
jsonpath "$.fruit[?(@.price.US==200)].name" equals "grape"