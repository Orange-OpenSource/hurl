# First, start a timer
GET http://localhost:8000/delay-init
HTTP 200

# This request must reach the server at least 1000ms after the first request
GET http://localhost:8000/delay
[Options]
delay: 1000ms
HTTP 200
