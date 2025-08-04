GET http://localhost:8000/cookies/error
HTTP 200
Set-Cookie: cookie1=value1; Expires=???; Secure; HttpOnly; Path=/
[Asserts]
cookie "cookie1[Expires]" exists
