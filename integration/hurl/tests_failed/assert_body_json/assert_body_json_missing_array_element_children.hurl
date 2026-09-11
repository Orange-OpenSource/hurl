# Child array element missing
GET http://localhost:8000/assert-body-json/missing_array_element_children
HTTP 200
[Asserts]
jsonpath "$.children[2]" == "Trevor"
