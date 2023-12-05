# Only variables with scalar value (string, integer, ...) can be rendered in template
GET http://localhost:8000/get-variables-not-renderable

HTTP 200
[Captures]
list: jsonpath "$.list"
object: jsonpath "$.object"
nodeset: jsonpath "$.nodeset" xpath "/"

GET http://localhost:8000/undefined
[QueryStringParams]
list: {{list}}
HTTP 200

GET http://localhost:8000/undefined
[QueryStringParams]
object: {{object}}
HTTP 200

GET http://localhost:8000/undefined
[QueryStringParams]
nodeset: {{nodeset}}
HTTP 200
