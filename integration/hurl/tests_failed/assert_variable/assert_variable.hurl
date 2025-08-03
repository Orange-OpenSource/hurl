GET http://localhost:8000/error-assert-variable
HTTP 200
[Captures]
status: status
length: header "content-length"
type: header "content-type"
[Asserts]
variable "toto" == "tata"
variable "status" == {{unknown}}
variable "status" == {{type}}
variable "status" == {{length}}


GET http://localhost:8000/error-assert-variable
[Options]
verbose: {{verbose}}
HTTP 200
