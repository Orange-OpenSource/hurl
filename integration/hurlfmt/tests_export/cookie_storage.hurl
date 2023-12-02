#
# experimental feature
# manage cookie store
#

# @cookie_storage_set: localhost    FALSE   /   FALSE   0   cookie1 valueA
GET http://localhost:8000/cookie-storage/assert-that-cookie1-is-valueA
HTTP 200

# @cookie_storage_clear
GET http://localhost:8000/cookie-storage/assert-that-cookie1-is-not-in-session
HTTP 200


