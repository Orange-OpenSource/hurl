GET http://localhost:8000/captures
HTTP 200
[Captures]
param1: header "header1"
param2: header "header2" regex "Hello (.*)!"
param3: header "header2" regex /Hello (.*)!/
# With the next captures, we test that the captured variable
# can be immediately used in the next captures + asserts
data1: body
data2: variable "data1"
[Asserts]
header "header1" == "{{param1}}"
variable "param1" == "value1"
variable "param2" == "Bob"
variable "param3" == "Bob"
variable "data2" == "Hello world!"


GET http://localhost:8000/captures-check
[QueryStringParams]
param1: {{param1}}
param2: {{param2}}
HTTP 200


GET http://localhost:8000/captures-check
[Query]
param1: {{param1}}
param2: {{param2}}
HTTP 200


GET http://localhost:8000/captures-xml
HTTP 200
[Captures]
a_node_set: xpath "//p"
[Asserts]
variable "a_node_set" count == 2


GET http://localhost:8000/captures-cookie
HTTP 200
[Captures]
a_cookie_value_attr: cookie "foo"
a_cookie_path_attr: cookie "foo[Path]"
a_cookie_expires_attr: cookie "foo[Expires]"
a_cookie_domain_attr: cookie "foo[Domain]"
a_cookie_max_age_attr: cookie "foo[Max-Age]"
a_cookie_http_only_attr: cookie "foo[HttpOnly]"
[Asserts]
variable "a_cookie_value_attr" == "value1"
variable "a_cookie_value_attr" == {{a_cookie_value_attr}}
variable "a_cookie_path_attr" == "/bar"
variable "a_cookie_path_attr" == {{a_cookie_path_attr}}
variable "a_cookie_expires_attr" format "%Y-%m-%dT%H:%M:%S%.fZ" == "2078-01-13T22:23:01Z"
variable "a_cookie_expires_attr" == {{a_cookie_expires_attr}}
variable "a_cookie_domain_attr" == "localhost"
variable "a_cookie_domain_attr" == {{a_cookie_domain_attr}}
variable "a_cookie_max_age_attr" == 2592000
variable "a_cookie_max_age_attr" == {{a_cookie_max_age_attr}}
variable "a_cookie_http_only_attr" exists



GET http://localhost:8000/captures-json
HTTP 200
[Captures]
an_object: jsonpath "$['an_object']"
a_list: jsonpath "$['a_list']"
a_null: jsonpath "$['a_null']"
an_integer: jsonpath "$['an_integer']"
a_big_integer: jsonpath "$['a_big_integer']"
a_float: jsonpath "$['a_float']"
a_bool: jsonpath "$['a_bool']"
a_string: jsonpath "$['a_string']"
a_date_like_string: jsonpath "$['a_date_like_string']"
a_date: variable "a_date_like_string" toDate "%Y-%m-%dT%H:%M:%S%.fZ"
some_bytes: bytes
all: jsonpath "$"
[Asserts]
variable "a_null" exists
variable "undefined" not exists
variable "a_null" == {{a_null}}
variable "an_integer" == {{an_integer}}
variable "a_big_integer" == {{a_big_integer}}
variable "a_float" == {{a_float}}
variable "a_bool" == {{a_bool}}
variable "a_string" == {{a_string}}
variable "a_string" == "{{a_string}}"
variable "a_list" == {{a_list}}
variable "a_date_like_string" == "2012-04-23T18:25:43.511Z"
variable "some_bytes" startsWith hex,7b22615f6e756c6c223a6e;



