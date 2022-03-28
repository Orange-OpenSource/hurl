GET http://localhost:8000/utf8

HTTP/1.0 200
[Asserts]
xpath "normalize-space(//data)" == "café"
xpath "normalize-space(//data)" == "caf\u{e9}"
