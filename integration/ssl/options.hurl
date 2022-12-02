GET https://localhost:8001/hello
[Options]
insecure: true

HTTP 200
`Hello World!`


GET https://localhost:8001/hello
[Options]
# Test that options overridden is possible, last is the winner.
insecure: false
insecure: true

HTTP 200
`Hello World!`


GET https://localhost:8001/hello
[Options]
cacert: ssl/cert.pem        # with a custom certificate

HTTP 200
`Hello World!`
