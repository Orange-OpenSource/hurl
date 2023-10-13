# This test is run when the libcurl used by Hurl
# doesn't support HTTP/3 so it should failed with
# an appropriate error message.
GET https://google.com
HTTP 200
