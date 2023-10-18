GET http://localhost:8000/ip
HTTP 200

GET http://localhost:8000/check-ipv4
[Options]
ipv4: true
HTTP 200
`127.0.0.1`

GET http://localhost:8000/check-ipv4
[Options]
ipv4: false
HTTP 400
`::1`

GET http://localhost:8000/check-ipv6
[Options]
ipv6: true
HTTP 200
`::1`

GET http://localhost:8000/check-ipv6
[Options]
ipv6: false
HTTP 400
`127.0.0.1`
