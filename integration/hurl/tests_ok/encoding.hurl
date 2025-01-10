# Even if the content is gzipped, body asserts `body, `bytes`, `sha256`, `md5`, `jsonpath` etc
# works on decompressed response.
# Note: the response body outputted on standard error remains however encoded (mainly to be consistent
# with curl)
GET http://localhost:8000/encoding/gzip
Accept-Encoding: gzip
HTTP 200
Content-Length: 32
Content-Encoding: gzip
Content-Type: text/html; charset=utf-8
[Asserts]
body == "Hello World!"
bytes == hex,48656c6c6f20576f726c6421;
sha256 == hex,7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069;
md5 == hex,ed076287532e86365e841e92bfc50d8c;
`Hello World!`
