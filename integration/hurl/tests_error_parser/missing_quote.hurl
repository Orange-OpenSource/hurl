POST http://localhost:8000/not-found
HTTP 404
[Asserts]
jsonpath "$.error.message" == "not found
