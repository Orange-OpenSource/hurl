# Uncompressed body tests:
# ---------------------------
# --compressed has no effect on non compressed.

GET http://localhost:8000/compressed/none
HTTP 200
Content-Length: 12
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`

# Test base64 implicit body:
GET http://localhost:8000/compressed/none
HTTP 200
base64,SGVsbG8gV29ybGQh;

# Test hex implicit body:
GET http://localhost:8000/compressed/none
HTTP 200
hex,48656c6c6f20576f726c6421;

# Test file implicit body:
GET http://localhost:8000/compressed/none
HTTP 200
file,hello.txt;


# GZip compressed body tests:
# ---------------------------

GET http://localhost:8000/compressed/gzip
HTTP 200
Content-Length: 32
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`

# Test base64 implicit body:
GET http://localhost:8000/compressed/gzip
HTTP 200
base64,SGVsbG8gV29ybGQh;

# Test hex implicit body:
GET http://localhost:8000/compressed/gzip
HTTP 200
hex,48656c6c6f20576f726c6421;

# Test file implicit body:
GET http://localhost:8000/compressed/gzip
HTTP 200
file,hello.txt;


# ZLib compressed body tests:
# ---------------------------

GET http://localhost:8000/compressed/zlib
HTTP 200
Content-Length: 20
Content-Encoding: deflate
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`

# Test base64 implicit body:
GET http://localhost:8000/compressed/zlib
HTTP 200
base64,SGVsbG8gV29ybGQh;

# Test hex implicit body:
GET http://localhost:8000/compressed/zlib
HTTP 200
hex,48656c6c6f20576f726c6421;

# Test file implicit body:
GET http://localhost:8000/compressed/zlib
HTTP 200
file,hello.txt;


# Brotli compressed body tests:
# ---------------------------

GET http://localhost:8000/compressed/brotli
HTTP 200
Content-Length: 17
Content-Encoding: br
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`

# Test base64 implicit body:
GET http://localhost:8000/compressed/brotli
HTTP 200
base64,SGVsbG8gV29ybGQh;

# Test hex implicit body:
GET http://localhost:8000/compressed/brotli
HTTP 200
hex,48656c6c6f20576f726c6421;

# Test file implicit body:
GET http://localhost:8000/compressed/brotli
HTTP 200
file,hello.txt;

# Test a large brotli compressed body:
GET http://localhost:8000/compressed/brotli_large
HTTP 200
Content-Encoding: br
Content-Type: image/jpeg
[Asserts]
bytes startsWith hex,ffd8ffe0;   # JPEG magic number
file,cat.jpg;

GET http://localhost:8000/compressed/brotli_identity
HTTP 200
Content-Length: 17
Content-Encoding: br, identity
Content-Type: text/html; charset=utf-8
[Asserts]
body startsWith "Hello"
bytes startsWith hex,48656c6c6f;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`
