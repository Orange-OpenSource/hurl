# Sample from https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PATCH
PATCH http://localhost:8000/patch/file.txt
Host: www.example.com
Content-Type: application/example
If-Match: "e0023aa4e"
HTTP 204
Content-Location: /file.txt
ETag: "e0023aa4f"
