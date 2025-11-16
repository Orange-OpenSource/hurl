# We test the [`--cookie-jar`](https://hurl.dev/docs/manual.html#cookie-jar) option
# that dumps the cookie storage to a file. Due to various support level among
# libcurl implementation for cookie secure export, we limit this test to non secure cookie.

GET http://localhost:8000/cookie-jar
HTTP 200
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Thu, 13 Jan 2078 22:23:01 GMT; HttpOnly; Path=/accounts
Set-Cookie: HSID=AYQEVnDKrdst; Domain=localhost; Expires=Thu, 13 Jan 2078 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=localhost; Expires=Thu, 13 Jan 2078 22:23:01 GMT; HttpOnly; Path=/
[Asserts]
header "Set-Cookie" count == 4
cookie "LSID" == "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Thu, 13 Jan 2078 22:23:01"
cookie "LSID[Max-Age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[Secure]" not exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[SameSite]" not exists
cookie "foo" == "\"a b c\""


GET http://localhost:8000/cookie-jar/hello
HTTP 200
