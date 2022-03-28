GET http://localhost:8000/capture-and-assert
HTTP/1.0 200
[Captures]
content_type: header "content-type"
[Asserts]
header "content-type" == "{{content_type}}"


