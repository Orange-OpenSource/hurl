GET http://localhost:8000/charset/default
HTTP/1.0 200
Content-Type: text/html; charset=utf-8
```<p>Hello World!</p>```

GET http://localhost:8000/charset/uppercase
HTTP/1.0 200
Content-Type: text/html; charset=UTF-8
[Asserts]
body == "<p>Hello World!</p>"
