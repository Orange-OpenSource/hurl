# Test body response with file assertion.
# We receive the body "Hello" and not "Hello World!"



GET http://localhost:8000/error-assert-file
HTTP 200
file,data.txt;

