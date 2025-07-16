GET http://localhost:8000/captures
HTTP 200
[Captures]
param1: header "header1"


GET http://localhost:8000/captures-xml
HTTP 200
[Captures]
a_node_set: xpath "//p"


GET http://localhost:8000/captures-cookie
HTTP 200
[Captures]
a_cookie_value_attr: cookie "foo"


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


GET http://localhost:8000/redirect-to-captures-json
[Options]
location: true
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
a_response: redirects nth 0


