#
# Experimental feature
# Manage cookie store
#

# Set a cookie in cookie jar (valid) for subsequent requests
# @cookie_storage_set: localhost    FALSE   /   FALSE   0   cookie1 valueA
GET http://localhost:8000/cookie-storage/assert-that-cookie1-is-valueA
HTTP 200

# This is noop (and should not crashed)
#@cookie_storage_set:
GET http://localhost:8000/cookie-storage/noop
HTTP 200

# @cookie_storage_clear
GET http://localhost:8000/cookie-storage/assert-that-cookie1-is-not-in-session
HTTP 200
