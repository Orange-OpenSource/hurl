GET http://localhost:8000/add-header
HTTP 200

GET http://localhost:8000/add-header-with-aggregation
header-a: foo
HTTP 200

GET http://localhost:8000/add-header-with-duplicate
header-b: bar
{"message":"hi!"}
HTTP 200
