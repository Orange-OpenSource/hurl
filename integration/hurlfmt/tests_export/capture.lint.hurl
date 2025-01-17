GET http://localhost:8000/dummy

HTTP 200
[Captures]
name: jsonpath "$.name"
token: jsonpath "$.token" redact
