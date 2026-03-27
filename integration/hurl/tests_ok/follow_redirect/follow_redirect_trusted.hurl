# Forward authorization header to a different host explicitly
GET http://localhost:8000/follow-redirect-basic-auth-trusted
Authorization: Basic Ym9iQGVtYWlsLmNvbTpzZWNyZXQ=
[Cookies]
fruit: lemon
HTTP 200
[Asserts]
redirects count == 1
redirects nth 0 location == "http://127.0.0.1:8000/followed-redirect-basic-auth-trusted"
url == "http://127.0.0.1:8000/followed-redirect-basic-auth-trusted"
header "Location" not exists
`Followed redirect Basic Auth!`
