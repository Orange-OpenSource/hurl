# Authentication with:
#   username=bob@email.com
#   password=secret

# the '@' must be url-escaped
GET http://bob%40email.com:secret@localhost:8000/basic-authentication-per-request
HTTP/1.0 200
```You are authenticated```

# Header value can be computed with `echo -n 'bob@email.com:secret' | base64`
GET http://localhost:8000/basic-authentication-per-request
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
HTTP/1.0 200
```You are authenticated```

# We can also use a BasicAuth section to specify user and password
# without any base64 encoding.
# Note that spaces surrounded user and password are trimmed. If you
# really want a space in your password (!!), you could use Hurl
# unicode literal \u{20}.
GET http://localhost:8000/basic-authentication-per-request
[BasicAuth]
bob@email.com: secret
HTTP/1.0 200
```You are authenticated```
