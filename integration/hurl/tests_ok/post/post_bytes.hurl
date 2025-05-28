POST http://localhost:8000/post-bytes
Content-Type: application/octet-stream
base64,AQID;  # echo -e -n '\x01\x02\x03' | base64
HTTP 200
