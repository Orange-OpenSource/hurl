# Request Cookie

GET http://localhost:8000/cookies/set-request-cookie1-valueA
[Cookies]
cookie1: valueA
HTTP/1.0 200

# The cookie is not added in the cookie storage
GET http://localhost:8000/cookies/assert-that-cookie1-is-not-in-session
HTTP/1.0 200

GET http://localhost:8000/cookies/set-multiple-request-cookies
[Cookies]
user1: Bob
user2: Bill
user3: {{name}}
HTTP/1.0 200


# Session Cookie

GET http://localhost:8000/cookies/set-session-cookie2-valueA
HTTP/1.0 200
[Asserts]
cookie "cookie2" == "valueA"

GET http://localhost:8000/cookies/assert-that-cookie2-is-valueA
HTTP/1.0 200

GET http://localhost:8000/cookies/assert-that-cookie2-is-valueA-and-valueB
[Cookies]
cookie2: valueB
HTTP/1.0 200


GET http://localhost:8000/cookies/delete-cookie2
HTTP/1.0 200
[Asserts]
cookie "cookie2" == ""
cookie "cookie2[Max-Age]" == 0

GET http://localhost:8000/cookies/assert-that-cookie2-is-not-in-session
HTTP/1.0 200

GET http://localhost:8000/cookies/set
HTTP/1.0 200
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/accounts
Set-Cookie: HSID=AYQEVnDKrdst; Domain=.localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=.localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/

[Asserts]
header "Set-Cookie" count == 3
cookie "LSID" equals "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[Expires]" == "Wed, 13 Jan 2021 22:23:01 GMT"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" not exists


