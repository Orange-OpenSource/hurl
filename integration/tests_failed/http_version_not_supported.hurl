# This test is run when the libcurl used by Hurl doesn't support HTTP/3 so it should failed with
# an appropriate error message.
GET http://localhost:8000/foo
HTTP 200
