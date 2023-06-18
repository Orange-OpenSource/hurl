GET http://localhost:8000/error-assert-template-variable-not-found
HTTP 200
[Asserts]
header "content-type" == "{{content_type}}"

