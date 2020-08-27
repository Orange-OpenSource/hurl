GET http://localhost:8000/error-assert-variable
HTTP/1.0 200
[Captures]
status: status
length: header "content-length"
type: header "content-type"
[Asserts]
variable "toto" equals "tata"
variable "status" equals {{unknown}}
variable "status" equals {{type}}
variable "status" equals {{length}}
