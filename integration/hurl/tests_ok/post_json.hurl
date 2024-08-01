POST http://localhost:8000/post-json
{
    "name": "Bob",
    "password": "&secret\\'<>",
    "age": 30,
    "strict": true,
    "spacing": "\n",
    "g_clef": "\uD834\uDD1E",
    "items": [true, "true", 1],
    "variable": "{{string_variable}}",
    "": "empty"
}
HTTP 200


POST http://localhost:8000/post-json-array
[1,2,3]
HTTP 200


POST http://localhost:8000/post-json-array-empty
[]
HTTP 200


POST http://localhost:8000/post-json-string
"Hello"
HTTP 200


POST http://localhost:8000/post-json-number
100
HTTP 200


POST http://localhost:8000/post-json-numbers
{
    "natural": 100,
    "negative": -1,
    "float": 3.333333333333333,
    "float_with_00": 123.00,
    "exponent": 100e100
}
HTTP 200


POST http://localhost:8000/post-json-boolean
true
HTTP 200

#
# Use variable in your input json
#
GET http://localhost:8000/get-name
HTTP 200
[Captures]
name: body


POST http://localhost:8000/check_name
{
    "name": "{{name}}"
}


