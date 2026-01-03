GET http://localhost:8000/rawbytes/empty
HTTP 200
Content-Type: application/octet-stream
[Asserts]
rawbytes isEmpty
rawbytes count == 0
