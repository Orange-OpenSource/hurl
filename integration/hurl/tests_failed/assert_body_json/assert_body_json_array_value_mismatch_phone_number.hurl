# First phone number modified
GET http://localhost:8000/assert-body-json/array_value_mismatch_phone_number
HTTP 200
[Asserts]
jsonpath "$.phone_numbers[0].number" == "212 555-1234"
