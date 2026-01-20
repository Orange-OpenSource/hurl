POST http://localhost:8000/post-multilines
```
name,age
bob,10
bill,22
```
HTTP 200


GET http://localhost:8000/get-bob-age
HTTP 200
[Captures]
bob_age: body


POST http://localhost:8000/post-multilines
```
name,age
bob,{{bob_age}}
bill,22
```
HTTP 200


POST http://localhost:8000/post-multilines-raw
```raw
name,age
bob,{{bob_age}}
bill,22
```
HTTP 200


# Hurl Unicode literals are not processed in multiline strings
POST http://localhost:8000/post-multilines-json
```json
{
    "g_clef": "\u{1D11E}"
}
```
HTTP 200
