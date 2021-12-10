POST http://localhost:8000/not-found
HTTP/1.1 404
[Asserts]
jsonpath "$.error.message" == "not found
