# Only variables with scalar value (string, integer, ...) can be rendered in template
GET http://localhost:8000/get-object

HTTP 200
[Captures]
object: jsonpath "$.values"

GET http://localhost:8000/undefined
[QueryStringParams]
param1: {{object}}
HTTP 200


GET http://localhost:8000/get-list

HTTP 200
[Captures]
list: jsonpath "$.values"

GET http://localhost:8000/undefined
[QueryStringParams]
param1: {{list}}
HTTP 200

