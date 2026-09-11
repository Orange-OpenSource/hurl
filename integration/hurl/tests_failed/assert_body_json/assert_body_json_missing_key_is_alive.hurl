# Is alive field deleted
GET http://localhost:8000/assert-body-json/missing_key_is_alive
HTTP 200
[Asserts]
jsonpath "$.is_alive" == true
