GET http://localhost:8000/empty_bytes
HTTP 200
Content-Type: application/octet-stream
[Asserts]
bytes isEmpty
bytes count == 0
