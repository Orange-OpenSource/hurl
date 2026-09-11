# Is alive field deleted
GET http://localhost:8000/assert-body-json/is_alive_deleted
HTTP 200
[Asserts]
jsonpath "$.is_alive" == true
