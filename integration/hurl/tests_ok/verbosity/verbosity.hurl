GET http://localhost:8000/verbosity
X-Foo: 12345678
HTTP 200
[Captures]
date: header "Date"


PUT http://localhost:8000/verbosity?id=bar
HTTP 405


POST http://localhost:8000/verbosity
{}
HTTP 200



