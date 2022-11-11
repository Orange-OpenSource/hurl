GET http://localhost:8000/get-list

HTTP 200
[Captures]
list: jsonpath "$.values"


GET http://localhost:8000/undefined
[QueryStringParams]
param1: {{list}}
HTTP 200

