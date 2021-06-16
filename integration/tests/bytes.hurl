GET http://localhost:8000/bytes
HTTP/1.0 200
Content-Type: application/octet-stream
[Asserts]
bytes equals hex,ff;
bytes countEquals 1

