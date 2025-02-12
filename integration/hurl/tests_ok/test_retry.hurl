# Get a successful response from a flaky API
GET http://localhost:8000/flaky
HTTP 200
[Asserts]
jsonpath "$.message" == "success"
