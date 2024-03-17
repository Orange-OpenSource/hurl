GET http://localhost:8000/charset/default
HTTP 200
Content-Type: text/html; charset=utf-8
`<p>Hello World!</p>`


GET http://localhost:8000/charset/latin1
HTTP 200
Content-Type: text/html; charset=latin1
[Asserts]
body == "<p>café</p>"


GET http://localhost:8000/charset/latin1/uppercase-key
HTTP 200
Content-Type: text/html; CHARSET=latin1
[Asserts]
body == "<p>café</p>"


GET http://localhost:8000/charset/gb2312
HTTP 200
Content-Type: text/html; charset=gb2312
[Asserts]
body == "<p>你好世界</p>"


GET http://localhost:8000/charset/cp1256
HTTP 200
Content-Type: text/html; charset=cp1256
[Asserts]
body == "<p>مرحبا بالعالم</p>"


GET http://localhost:8000/charset/uppercase-value
HTTP 200
Content-Type: text/html; charset=UTF-8
[Asserts]
body == "<p>Hello World!</p>"


GET http://localhost:8000/charset/many-keys
HTTP 200
Content-Type: text/plain; version=0.0.4; charset=utf-8; escaping=values
[Asserts]
body == "<p>Hello World!</p>"
