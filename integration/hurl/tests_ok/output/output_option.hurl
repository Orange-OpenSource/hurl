POST http://localhost:8000/output/endpoint1
[Options]
output: ../build/output_request_1.bin
{ "user": "bob" }
HTTP 200


# output options supports templating
GET http://localhost:8000/output/endpoint2
[Options]
variable: filename=output_request_2
output: ../build/{{filename}}.bin
HTTP 200


GET http://localhost:8000/output/endpoint3
HTTP 200
