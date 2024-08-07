# First, start a timer
GET http://localhost:8000/delay-init
HTTP 200


# This request must reach the server at least 1000ms after the previous request
GET http://localhost:8000/delay
HTTP 200


# This request must reach the server at least 1000ms after the previous request
GET http://localhost:8000/delay
HTTP 200


# This request must reach the server at least 1000ms after the previous request
# We artificially trigger retry to test that the delay is NOT add for each retry.
GET http://localhost:8000/delay-and-retry
[Options]
retry: 10
retry-interval: 10ms
HTTP 200
[Asserts]
body toInt > 5 # Trigger a retry for 5 requests

