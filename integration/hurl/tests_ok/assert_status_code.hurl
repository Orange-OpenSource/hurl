GET http://localhost:8000/assert-status-code
HTTP 201


# Assert HTTP 1.1
GET http://localhost:8000/assert-status-code
HTTP/1.1 201


# Simply check that the status code is not 200
# Do not run implicit assert of HTTP response version and code
GET http://localhost:8000/assert-status-code
HTTP *
[Asserts]
status != 200


# Simply check that the status code is OK
GET http://localhost:8000/assert-status-code
HTTP *
[Asserts]
status >= 200
status < 300
