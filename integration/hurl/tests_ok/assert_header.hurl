GET http://localhost:8000/assert-header
HTTP 200
Content-Type: text/html; charset=utf-8
Set-Cookie: cookie1=value1; Path=/
Set-Cookie: cookie2=value2; Path=/
[Asserts]
header "Custom" not exists
header "Content-Type" exists
header "Header1" == "value1"
header "ETag" == "\"33a64df551425fcc55e4d42a148795d9f25f89d4\""
header "Expires" == "Wed, 21 Oct 2015 07:28:00 GMT"
header "Expires" toDate "%a, %d %b %Y %H:%M:%S GMT" format "%Y" == "2015"
header "Expires" toDate "%a, %d %b %Y %H:%M:%S GMT" daysBeforeNow > 1000
header "Set-Cookie" exists
header "Set-Cookie" count == 3
header "Set-Cookie" includes "cookie1=value1; Path=/"
header "Set-Cookie" not includes "cookie4=value4; Path=/"
header "X-Fruit" isCollection
header "x-fruit" count == 4
header "X-Fruit" nth 0 == "Banana"
header "x-fruit" nth 1 == "Lemon"
header "X-Fruit" nth 2 != "Lemon"
header "X-Fruit" nth 2 == "Grape"
header "X-Fruit" nth 3 == "Strawberry"
header "X-Fruit" includes "Strawberry"


# Same request as previously, but now we check that we can capture the header "X-Fruit" collection as a variable
# before asserting it s values.
GET http://localhost:8000/assert-header
HTTP 200
[Captures]
fruits: header "X-fruit"
[Asserts]
header "X-Fruit" isCollection
variable "fruits" isCollection
variable "fruits" count == 4
variable "fruits" nth 0 == "Banana"
variable "fruits" nth 3 == "Strawberry"
