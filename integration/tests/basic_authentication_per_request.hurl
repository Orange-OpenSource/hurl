GET http://bob:secret@localhost:8000/basic-authentication-per-request
HTTP/1.0 200
```You are authenticated```

GET http://localhost:8000/basic-authentication-per-request
Authorization: Basic Ym9iOnNlY3JldA==
HTTP/1.0 200
```You are authenticated```

GET http://localhost:8000/basic-authentication-per-request
[BasicAuth]
bob:secret
HTTP/1.0 200
```You are authenticated```
