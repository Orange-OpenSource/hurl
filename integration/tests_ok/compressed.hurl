# -- COMPRESSED HAS NO EFFECT ON NON-COMPRESSED
GET http://localhost:8000/compressed/none
HTTP/1.0 200
Content-Length: 12
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```Hello World!```

GET http://localhost:8000/compressed/gzip
HTTP/1.0 200
Content-Length: 32
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```Hello World!```

GET http://localhost:8000/compressed/zlib
HTTP/1.0 200
Content-Length: 20
Content-Encoding: deflate
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```Hello World!```

GET http://localhost:8000/compressed/brotli
HTTP/1.0 200
Content-Length: 17
Content-Encoding: br
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```Hello World!```

GET http://localhost:8000/compressed/brotli_identity
HTTP/1.0 200
Content-Length: 17
Content-Encoding: br, identity
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
```Hello World!```
