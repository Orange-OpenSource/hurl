GET http://localhost:8000/assert-regex
HTTP 200
[Asserts]
regex "Hello ([0-9]+)!" not exists
regex /Hello ([0-9]+)!/ not exists
body regex /Hello ([0-9]+)!/ not exists
regex "Hello ([a-zA-Z]+)!" == "World"
regex /Hello ([a-zA-Z]+)!/ == "World"
