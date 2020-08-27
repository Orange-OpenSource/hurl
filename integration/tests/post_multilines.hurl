POST http://localhost:8000/post-multilines
```
name,age
bob,10
bill,22
```

HTTP/1.0 200


GET http://localhost:8000/get-bob-age
HTTP/1.0 200
[Captures]
bob_age: body


POST http://localhost:8000/post-multilines
```
name,age
bob,{{bob_age}}
bill,22
```
HTTP/1.0 200
