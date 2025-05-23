GET http://localhost:8000/dummy_bytes
HTTP 200
[Asserts]
duration < 10000


GET http://localhost:8000/dummy_bytes
[Options]
# A 30,000,000 bytes limited at 2,000,000 bytes/s is roughly 15s.
# A check on [10s, 20s] is enough, with a good margin of error.
limit-rate: 2000000
HTTP 200
[Asserts]
duration >= 10000
duration <= 20000
