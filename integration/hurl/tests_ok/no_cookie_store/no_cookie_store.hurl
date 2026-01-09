GET http://localhost:8000/no_cookie_store/set
HTTP 200
[Asserts]
header "Set-Cookie" exists
header "Set-cookie" == "foo=1234; Expires=Mon, 12 Jan 2026 22:23:01 GMT; Path=/no_cookie/check"
# Even with --no-cookie `cookie` assert is working as it's syntax sugar over
# `Set-Cookie` response header .
cookie "foo" == "1234"
cookie "foo[Path]" == "/no_cookie/check"
cookie "foo[Expires]" dateFormat "%a, %d %b %Y %H:%M:%S" == "Mon, 12 Jan 2026 22:23:01"


GET http://localhost:8000/no_cookie_store/check
HTTP 200


GET http://localhost:8000/no_cookie_store/request_with_cookie
[Cookies]
color: blue
HTTP 200
