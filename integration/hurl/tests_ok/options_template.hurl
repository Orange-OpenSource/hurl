# Reuse existing tests with options
# using template

GET http://localhost:8000/redirect-absolute
[Options]
variable: redirect=true
location: {{redirect}}

GET http://localhost:8000/delay-init
HTTP 200

GET http://localhost:8000/delay # This request must reach the server at least 1000ms after the previous request
[Options]
variable: duration=1000
delay: {{duration}}
HTTP 200

GET http://bar.com:8000/resolve
[Options]
variable: port=8000
resolve: bar.com:{{port}}:127.0.0.1
HTTP 200
`Hello World!`
