POST http://localhost:8000/post-json

{
    "name": "Bob",
    "password": "secret"
}

HTTP/1.0 200

POST http://localhost:8000/post-json-array
[1,2,3]

HTTP/1.0 200

POST http://localhost:8000/post-json-string
"Hello"

HTTP/1.0 200

POST http://localhost:8000/post-json-number
100

HTTP/1.0 200

POST http://localhost:8000/post-json-numbers
{
    "natural": 100,
    "negative": -1,
    "float": "3.333333333333333",
    "exponent": 100e100
}


HTTP/1.0 200

POST http://localhost:8000/post-json-boolean
true

HTTP/1.0 200

#
# Use variable in your input json
#
GET http://localhost:8000/get-name
HTTP/1.0 200
[Captures]
name: body

POST http://localhost:8000/post-json

{
    "name": "{{name}}",
    "password": "secret"
}