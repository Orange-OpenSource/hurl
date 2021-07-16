GET http://localhost:8000/predicate/error/type
HTTP/1.0 200
[Asserts]
jsonpath "$.status" equals "true"
#jsonpath "$.count" equals "0"
jsonpath "$.count" equals 0
jsonpath "$.message" equals 0
jsonpath "$.empty" equals 0
jsonpath "$.number" equals 1.1
jsonpath "$.message" startsWith "hi"
jsonpath "$.message" endsWith "hi"
jsonpath "$.message" contains "hi"
jsonpath "$.message" matches "hi"
jsonpath "$.message" count == 1
jsonpath "$.toto" exists
jsonpath "$.message" not exists
jsonpath "$.list" count == 2