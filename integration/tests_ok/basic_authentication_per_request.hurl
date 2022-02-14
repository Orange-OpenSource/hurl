GET http://bob:secret@localhost:8000/basic-authentication-per-request
HTTP/1.0 200
```You are authenticated```

GET http://localhost:8000/basic-authentication-per-request
Authorization: Basic Ym9iOnNlY3JldA==       # Header value can be computed with `echo -n 'bob:secret' | base64`
HTTP/1.0 200
```You are authenticated```

# We can also use a BasicAuth section to specify user and password
# without any base64 encoding.
# Note that spaces surrounded user and password are trimmed. If you
# really want a space in your password (!!), you could use Hurl
# unicode literal \u{20}.
GET http://localhost:8000/basic-authentication-per-request
[BasicAuth]
bob: secret
HTTP/1.0 200
```You are authenticated```
