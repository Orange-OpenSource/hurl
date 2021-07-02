GET http://localhost:8000/bytes
HTTP/1.0 200
Content-Type: application/octet-stream
[Asserts]
bytes equals hex,ff;
bytes equals base64,/w==;
bytes count == 1
sha256 equals hex,a8100ae6aa1940d0b663bb31cd466142ebbdbd5187131b92d93818987832eb89;
