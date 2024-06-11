GET http://localhost:8000/cookies/set
HTTP 200
Set-Cookie: LSID=DQAAAKEaem_vYg; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/accounts
Set-Cookie: HSID=AYQEVnDKrdst; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; HttpOnly; Path=/
Set-Cookie: SSID=Ap4PGTEq; Domain=localhost; Expires=Wed, 13 Jan 2021 22:23:01 GMT; Secure; HttpOnly; Path=/
[Asserts]
header "Set-Cookie" count == 3
# Cookies attributes are case insensitive (the lowercase attribute are not accepted by hurlfmt)
cookie "LSID" == "DQAAAKEaem_vYg"
cookie "LSID[Value]" == "DQAAAKEaem_vYg"
cookie "LSID[Expires]" exists
cookie "LSID[expires]" exists
cookie "LSID[Expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "LSID[expires]" format "%a, %d %b %Y %H:%M:%S" == "Wed, 13 Jan 2021 22:23:01"
cookie "LSID[Max-Age]" not exists
cookie "LSID[max-age]" not exists
cookie "LSID[Domain]" not exists
cookie "LSID[domain]" not exists
cookie "LSID[Path]" == "/accounts"
cookie "LSID[path]" == "/accounts"
cookie "LSID[Secure]" exists
cookie "LSID[secure]" exists
cookie "LSID[HttpOnly]" exists
cookie "LSID[httponly]" exists
cookie "LSID[SameSite]" not exists
cookie "LSID[samesite]" not exists
