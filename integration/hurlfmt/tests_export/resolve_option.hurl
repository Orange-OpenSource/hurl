# --resolve option allow to us custom address for a specific host and port pair.
GET http://foo.com:8000/resolve
[Options]
resolve: foo.com:8000:127.0.0.1
HTTP 200
`Hello World!`


GET http://bar.com:8000/resolve
[Options]
resolve: bar.com:8000:127.0.0.1
HTTP 200
`Hello World!`


GET http://baz.com:8000/resolve
[Options]
resolve: baz.com:8000:127.0.0.1
HTTP 200
`Hello World!`
