GET http://localhost:8000/rawbytes
HTTP 200
Content-Type: application/octet-stream
[Asserts]
rawbytes == hex,010203;
rawbytes == base64,AQID;
rawbytes count == 3
rawbytes startsWith hex,01;
rawbytes endsWith hex,03;
rawbytes contains hex,02;

GET http://localhost:8000/rawbytes/gzip
HTTP 200
Content-Encoding: gzip
[Asserts]
bytes startsWith hex,48656c6c6f;
rawbytes startsWith hex,1f8b;
rawbytes count == 32
