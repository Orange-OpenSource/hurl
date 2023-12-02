# Test the option --output (see <https://hurl.dev/docs/manual.html#output>)
POST http://localhost:8000/output/endpoint1
{ "user": "bob" }
HTTP 200


GET http://localhost:8000/output/endpoint2
HTTP 200
