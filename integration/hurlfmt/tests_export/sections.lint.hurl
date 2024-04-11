GET http://localhost:8000/hello
[Options]
verbose: true
[QueryStringParams]
order: newest
search: {{custom-search}}
count: 100
[BasicAuth]
bob: secret
[FormParams]
default: false
token: {{token}}
email: john.doe@rookie.org
number: 33611223344
[MultipartFormData]
field1: value1
field2: file,example.txt;
field3: file,example.zip; application/zip
[Cookies]
theme: light
sessionToken: abc123
HTTP 200
[Captures]
count: jsonpath "$.count"
[Asserts]
variable "count" == 1

`Hello World!`

# Empty sections
GET http://localhost:8000/hello
[Options]
[QueryStringParams]
# FIXME: [BasicAuth] section can't be empty
#[BasicAuth]
[FormParams]
[MultipartFormData]
[Cookies]
HTTP 200
[Captures]
[Asserts]
`Hello World!`
