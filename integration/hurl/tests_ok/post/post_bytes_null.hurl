POST http://localhost:8000/post-bytes-null
Content-Type: application/octet-stream
base64,AAECAw==;  # printf '\x00\x01\x02\x03' | base64
HTTP 200
