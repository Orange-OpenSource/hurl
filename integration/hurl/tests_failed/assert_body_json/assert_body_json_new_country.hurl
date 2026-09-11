# New country field added
GET http://localhost:8000/assert-body-json/new_country
HTTP 200
[Asserts]
jsonpath "$.country" not exists
# Semantic assert body JSON not implemented yet
# {
#   "first_name": "John",
#   "last_name": "Smith",
#   "is_alive": true,
#   "age": 22,
#   "address": {
#     "street_address": "21 2nd Street",
#     "city": "New York",
#     "state": "NY",
#     "postal_code": "10021-3100"
#   },
#   "phone_numbers": [
#     {
#       "type": "home",
#       "number": "212 555-1234"
#     },
#     {
#       "type": "office",
#       "number": "646 555-4567"
#     }
#   ],
#   "children": [
#     "Catherine",
#     "Thomas",
#     "Trevor"
#   ],
#   "spouse": null
# }
