GET http://localhost:8000/undefined
HTTP 200

GET http://localhost:8000/undefined
HTTP *
[Asserts]
status == 200
