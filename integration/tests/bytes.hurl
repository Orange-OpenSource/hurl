GET http://localhost:8000/bytes
HTTP/1.0 200
Content-Type: application/octet-stream
[Asserts]
bytes equals hex,010203;
bytes equals base64,AQID;
bytes count == 3
bytes startsWith hex,01;
bytes endsWith hex,03;
bytes contains hex,02;
sha256 equals hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
md5 equals hex,5289df737df57326fcdd22597afb1fac;
