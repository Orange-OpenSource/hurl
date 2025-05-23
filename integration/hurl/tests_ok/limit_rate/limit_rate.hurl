GET http://localhost:8000/dummy_bytes
# A ~2.8M live file for test
# GET https://upload.wikimedia.org/wikipedia/commons/4/4d/Cat_November_2010-1a.jpg
HTTP 200
[Asserts]
# A 30,000,000 bytes limited at 2,000,000 bytes/s is roughly 15s.
# A check on [10s, 20s] is enough, with a good margin of error.
duration >= 10000
duration <= 20000
