GET http://localhost:8000/very-verbose/redirect
HTTP/* 200

GET http://localhost:8000/very-verbose/encoding/latin1
HTTP/* 200

POST http://localhost:8000/very-verbose/compressed/brotli
Accept-Encoding: brotli
HTTP/* 200

GET http://localhost:8000/very-verbose/cat
HTTP/* 200

GET http://localhost:8000/very-verbose/done
x-foo: bar
HTTP/* 200
