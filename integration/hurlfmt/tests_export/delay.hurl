# First, start a timer
GET http://localhost:8000/delay-init
HTTP 200

# This request must reach the server at least 1000ms after the previous request
GET http://localhost:8000/delay
HTTP 200

# This request must reach the server at least 1000ms after the previous request
GET http://localhost:8000/delay
HTTP 200
