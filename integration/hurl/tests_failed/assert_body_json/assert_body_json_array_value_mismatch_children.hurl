# Child array value mismatch
GET http://localhost:8000/assert-body-json/array_value_mismatch_children
HTTP 200
[Asserts]
jsonpath "$.children[0]" == "Catherine"
