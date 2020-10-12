GET http://localhost:8000/compressed/gzip
HTTP/1.0 200
Content-Length: 32
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
```Hello World!```

# -- COMPRESSED HAS NO EFFECT ON NON-COMPRESSED
GET http://localhost:8000/compressed/none
HTTP/1.0 200
Content-Length: 12
Content-Type: text/html; charset=utf-8
```Hello World!```