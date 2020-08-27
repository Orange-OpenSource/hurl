GET http://localhost:8000/captures

HTTP/1.0 200
[Captures]
param1: header "header1"
param2: header "header2" regex "Hello (.*)!"
[Asserts]
variable "param1" equals "value1"
variable "param2" equals "Bob"

GET http://localhost:8000/captures-check
[QueryStringParams]
param1: {{param1}}
param2: {{param2}}

HTTP/1.0 200


GET http://localhost:8000/captures-json
HTTP/1.0 200
[Captures]
an_object:  jsonpath "$['an_object']"
a_list:     jsonpath "$['a_list']"
a_null:     jsonpath "$['a_null']"
an_integer: jsonpath "$['an_integer']"
a_float:    jsonpath "$['a_float']"
a_bool:     jsonpath "$['a_bool']"
a_string:   jsonpath "$['a_string']"
all:        jsonpath "$"
[Asserts]
variable "a_null" exists
variable "undefined" not exists
variable "a_null" equals {{a_null}}
variable "an_integer" equals {{an_integer}}
variable "a_float" equals {{a_float}}
variable "a_bool" equals {{a_bool}}
variable "a_string" equals {{a_string}}
variable "a_list" equals {{a_list}}


