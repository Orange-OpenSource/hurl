GET http://localhost:8000/dummy_bytes
# A ~2.8M live file for test
# GET https://upload.wikimedia.org/wikipedia/commons/4/4d/Cat_November_2010-1a.jpg
HTTP 200
[Asserts]
duration >= 10000
duration <= 20000
