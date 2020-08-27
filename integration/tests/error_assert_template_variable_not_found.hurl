GET http://localhost:8000/error-assert-template-variable-not-found
HTTP/1.0 200
[Asserts]
header "content-type" equals "{{content_type}}"

