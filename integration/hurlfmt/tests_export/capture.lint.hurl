GET http://localhost:8000/dummy

HTTP 200
[Captures]
name: jsonpath "$.name"
name2: jsonpath "$.name"          # name
token: jsonpath "$.token" redact
token2: jsonpath "$.token" redact # token
