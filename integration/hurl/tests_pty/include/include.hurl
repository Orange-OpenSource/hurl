GET http://localhost:8000/pty/include
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.strings.english" == "Hello, world!"
