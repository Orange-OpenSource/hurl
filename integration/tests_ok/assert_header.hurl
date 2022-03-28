GET http://localhost:8000/assert-header
HTTP/1.0 200
Content-Type: text/html; charset=utf-8
Set-Cookie: cookie1=value1; Path=/
Set-Cookie: cookie2=value2; Path=/
[Asserts]
header "Custom" not exists
header "Content-Type" exists
header "Header1" == "value1"
header "ETag" == "\"33a64df551425fcc55e4d42a148795d9f25f89d4\""
header "Set-Cookie" exists
header "Set-Cookie" count == 3
header "Set-Cookie" includes "cookie1=value1; Path=/"
header "Set-Cookie" not includes "cookie4=value4; Path=/"