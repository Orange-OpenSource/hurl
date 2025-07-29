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
cookie "cookie2[Expires]" not exists


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
cookie "LSID[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" not exists

# Check that cookies attributes are case-insensitive
GET http://localhost:8000/cookies/attributes/caseinsentitive
HTTP 200
[Asserts]
header "Set-Cookie" nth 0 == "CAMELCASE=value; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Max-Age=1000; Path=/accounts; SECURE; HttpOnly; SameSite=Strict"
cookie "CAMELCASE[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "CAMELCASE[Max-Age]" == 1000
cookie "CAMELCASE[Path]" == "/accounts"
cookie "CAMELCASE[HttpOnly]" exists
cookie "CAMELCASE[Secure]" exists
cookie "CAMELCASE[SameSite]" == "Strict"

header "Set-Cookie" nth 1 == "LOWERCASE=value; expires=Wed, 13 Jan 2021 22:23:01 GMT; max-age=1000; path=/accounts; SECURE; httponly; samesite=Strict"
cookie "LOWERCASE[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "LOWERCASE[Max-Age]" == 1000
cookie "LOWERCASE[Path]" == "/accounts"
cookie "LOWERCASE[Secure]" exists
cookie "LOWERCASE[HttpOnly]" exists
cookie "LOWERCASE[SameSite]" == "Strict"

header "Set-Cookie" nth 2 == "UPPERCASE=value; EXPIRES=Wed, 13 Jan 2021 22:23:01 GMT; MAX-AGE=1000; PATH=/accounts; SECURE; HTTPONLY; SAMESITE=Strict"
cookie "UPPERCASE[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "UPPERCASE[Max-Age]" == 1000
cookie "UPPERCASE[Path]" == "/accounts"
cookie "UPPERCASE[Secure]" exists
cookie "UPPERCASE[HttpOnly]" exists
cookie "UPPERCASE[SameSite]" == "Strict"

# Check that expires attribute value is parsed successfully
GET http://localhost:8000/cookies/expires
HTTP 200
Set-Cookie: expires_standard=value; Expires=Wed, 13 Jan 2021 22:23:01 GMT;
Set-Cookie: expires_with_dash=value; Expires=Wed, 13-Jan-2021 22:23:01 GMT;
[Asserts]
cookie "expires_standard[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "expires_with_dash[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
