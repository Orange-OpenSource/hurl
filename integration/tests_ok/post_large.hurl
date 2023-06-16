# Post a relatively large payload (~ 15M)
POST http://localhost:8000/post_large
Content-Type: application/octet-stream
file,post_large.bin;
HTTP 200
[Asserts]
body toInt == 15728640
