GET http://localhost:8000/predicates-string

HTTP/1.0 200
[Asserts]
body == "Hello World!"
body startsWith "Hello"
body endsWith "!"
body contains "llo"
body matches "Hello [a-zA-Z]+!"
body matches /Hello [a-zA-Z]+!/


GET http://localhost:8000/predicates-string-empty
HTTP/1.0 200
[Asserts]
body == ""
body exists

GET http://localhost:8000/predicates-string-unicode
HTTP/1.0 200
[Asserts]
body == "\u{2708}"
bytes count equals 3
