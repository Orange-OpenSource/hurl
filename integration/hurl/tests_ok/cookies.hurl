# Request Cookie

GET http://localhost:8000/cookies/set-request-cookie1-valueA
[Cookies]
cookie1: valueA
HTTP 200


# The cookie is not added in the cookie storage
GET http://localhost:8000/cookies/assert-that-cookie1-is-not-in-session
HTTP 200


GET http://localhost:8000/cookies/set-multiple-request-cookies
[Cookies]
user1: Bob
user2: Bill
user3: {{name}}
HTTP 200


# Session Cookie

GET http://localhost:8000/cookies/set-session-cookie2-valueA
HTTP 200
[Asserts]
cookie "cookie1" not exists
cookie "cookie2" == "valueA"
cookie "cookie2[Path]" == "/"
# See https://github.com/Orange-OpenSource/hurl/issues/2870
#cookie "cookie2[Expires]" not exists


GET http://localhost:8000/cookies/assert-that-cookie2-is-valueA
HTTP 200


GET http://localhost:8000/cookies/assert-that-cookie2-is-valueA-and-valueB
[Cookies]
cookie2: valueB
HTTP 200


GET http://localhost:8000/cookies/delete-cookie2
HTTP 200
[Asserts]
cookie "cookie2" == ""
cookie "cookie2[Max-Age]" == 0


GET http://localhost:8000/cookies/assert-that-cookie2-is-not-in-session
HTTP 200


GET http://localhost:8000/cookies/set
HTTP 200
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/accounts
Set-Cookie: HSID=AYQEVnDKrdst; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/
[Asserts]
header "Set-Cookie" count == 3
cookie "LSID" == "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[Expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" not exists


