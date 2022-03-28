GET http://localhost:8000/assert-status-code
HTTP/1.0 201

# simply check that the status code is not 200
# do not run implicit assert of http response version and code
GET http://localhost:8000/assert-status-code
HTTP/* *
[Asserts]
status != 200


# simply check that the status code is OK
GET http://localhost:8000/assert-status-code
HTTP/* *
[Asserts]
status >= 200
status < 300