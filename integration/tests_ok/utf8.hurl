GET http://localhost:8000/utf8

HTTP 200
Content-Type: text/html; charset=utf-8
Content-Length: 18
[Asserts]
xpath "normalize-space(//data)" == "café"
xpath "normalize-space(//data)" == "caf\u{e9}"
bytes == hex,3c646174613e636166c3a93c2f646174613e;