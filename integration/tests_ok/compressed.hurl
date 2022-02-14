# -- COMPRESSED HAS NO EFFECT ON NON-COMPRESSED
GET http://localhost:8000/compressed/none
HTTP/1.0 200
Content-Length: 12
Content-Type: text/html; charset=utf-8
```Hello World!```

GET http://localhost:8000/compressed/gzip
HTTP/1.0 200
Content-Length: 32
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
```Hello World!```

GET http://localhost:8000/compressed/zlib
HTTP/1.0 200
Content-Length: 20
Content-Encoding: deflate
Content-Type: text/html; charset=utf-8
```Hello World!```

GET http://localhost:8000/compressed/brotli
HTTP/1.0 200
Content-Length: 17
Content-Encoding: br
Content-Type: text/html; charset=utf-8
```Hello World!```

GET http://localhost:8000/compressed/brotli_identity
HTTP/1.0 200
Content-Length: 17
Content-Encoding: br, identity
Content-Type: text/html; charset=utf-8
```Hello World!```