# Child array element unexpected
GET http://localhost:8000/assert-body-json/unexpected_array_element_children
HTTP 200
[Asserts]
jsonpath "$.children[3]" not exists
