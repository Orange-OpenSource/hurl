GET http://localhost:8000/encoding/utf8
HTTP/1.0 200
Content-Type: text/html; charset=utf-8
[Asserts]
body equals "caf\u{e9}"


GET http://localhost:8000/encoding/latin1
HTTP/1.0 200
Content-Type: text/html; charset=ISO-8859-1
[Asserts]
body equals "caf\u{e9}"


