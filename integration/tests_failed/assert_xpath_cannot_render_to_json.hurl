GET http://localhost:8000/error-xpath-to-json
HTTP 200
[Captures]
head: xpath "//html/head"

POST http://localhost:8000/undefined
{
    "head": {{head}}
}
