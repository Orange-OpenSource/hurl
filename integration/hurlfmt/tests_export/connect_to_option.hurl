GET http://foo.com/connect-to
[Options]
connect-to: foo.com:80:localhost:8000
HTTP 200
`Hello World!`


GET http://bar.com/connect-to
[Options]
# We test that we can define multiple connect-to options.
connect-to: foo.com:80:localhost:8000
connect-to: bar.com:80:localhost:8000
connect-to: baz.com:80:localhost:8000
HTTP 200
`Hello World!`


GET http://baz.com/connect-to
[Options]
connect-to: foo.com:80:localhost:8000
connect-to: bar.com:80:localhost:8000
connect-to: baz.com:80:localhost:8000
HTTP 200
`Hello World!`
