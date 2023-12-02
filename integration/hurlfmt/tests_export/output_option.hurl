POST http://localhost:8000/output/endpoint1
[Options]
output: build/output_request_1.bin
{ "user": "bob" }
HTTP 200

GET http://localhost:8000/output/endpoint2
HTTP 200
