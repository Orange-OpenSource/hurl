GET http://localhost:8000/large

HTTP/1.0 200
Content-Type: application/octet-stream
Content-Length: 536870912

[Asserts]
duration <= 30000   # Duration in ms



