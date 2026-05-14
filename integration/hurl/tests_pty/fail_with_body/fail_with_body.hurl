GET http://localhost:8000/pty/fail_with_body
HTTP 200
[Asserts]
jsonpath "$.fruits" count == 12
jsonpath "$.fruits[0].name" == "Lemon"
