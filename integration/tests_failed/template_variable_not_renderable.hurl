GET http://localhost:8000/get-list
HTTP/1.0 200
[Captures]
list: jsonpath "$.values"

GET http://localhost:8000/undefined
[QueryStringParams]
param1: {{list}}
HTTP/1.1 200

