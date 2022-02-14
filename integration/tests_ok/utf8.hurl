GET http://localhost:8000/utf8

HTTP/1.0 200
[Asserts]
xpath "normalize-space(//data)" equals "café"
xpath "normalize-space(//data)" equals "caf\u{e9}"
