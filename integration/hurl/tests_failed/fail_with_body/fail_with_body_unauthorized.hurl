GET http://localhost:8000/fail_with_body
[Options]
output: ../out.json
HTTP 200
[Asserts]
jsonpath "$.fruits" count == 12
jsonpath "$.fruits[0].name" == "Lemon"
