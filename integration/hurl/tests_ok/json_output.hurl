# A test file with asserts, captures, redirection
# to test structured JSON output (with --json).


# Simple GET
GET http://localhost:8000/hello
HTTP 200
`Hello World!`


# Cookies!
GET http://localhost:8000/cookies/set-session-cookie2-valueA
HTTP 200
[Asserts]
cookie "cookie2" == "valueA"


# No redirect
GET http://localhost:8000/follow-redirect
Accept: text/plain
HTTP 302
[Asserts]
header "Location" == "http://localhost:8000/following-redirect"


# With redirect
GET http://localhost:8000/follow-redirect
Accept: text/plain
[Options]
location: true
HTTP 200
[Asserts]
header "Location" not exists
`Followed redirect!`


# Some Captures
GET http://localhost:8000/captures
HTTP 200
[Captures]
param1: header "header1"
param2: header "header2" regex "Hello (.*)!"
param3: header "header2" regex /Hello (.*)!/
data1: body
data2: variable "data1"


# Some JSON asserts
GET http://localhost:8000/assert-json
HTTP 200
[Asserts]
jsonpath "$.errors[1].id" == "error2"
jsonpath "$.tags" includes "test"
jsonpath "$.tags" not includes "prod"
jsonpath "$.tags" not includes null


