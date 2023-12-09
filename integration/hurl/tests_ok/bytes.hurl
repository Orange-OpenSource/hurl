GET http://localhost:8000/bytes
HTTP 200
Content-Type: application/octet-stream
[Asserts]
bytes == hex,010203;
bytes == base64,AQID;
bytes == file,bytes.bin;
bytes count == 3
bytes startsWith hex,01;
bytes endsWith hex,03;
bytes contains hex,02;
sha256 == hex,039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81;
md5 == hex,5289df737df57326fcdd22597afb1fac;
