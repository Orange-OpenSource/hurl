GET http://localhost:8000/predicate/error/type
HTTP 200
[Asserts]
jsonpath "$.status" == "true"
#jsonpath "$.count" equals "0"
jsonpath "$.count" == 0
jsonpath "$.message" == 0
jsonpath "$.empty" == 0
jsonpath "$.number" == 1.1
jsonpath "$.message" startsWith "hi"
jsonpath "$.message" endsWith "hi"
jsonpath "$.message" contains "hi"
jsonpath "$.message" matches "hi"
jsonpath "$.message" count == 1
jsonpath "$.toto" exists
jsonpath "$.message" not exists
jsonpath "$.list" count == 2
