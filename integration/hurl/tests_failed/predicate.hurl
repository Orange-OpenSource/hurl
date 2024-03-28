GET http://localhost:8000/predicate/error/type
HTTP 200
[Asserts]
jsonpath "$.status" == "true"
jsonpath "$.count" == 0
jsonpath "$.message" == 0
jsonpath "$.empty" == 0
jsonpath "$.number" == 1.1
jsonpath "$.count" startsWith "0"
jsonpath "$.count" endsWith "0"
jsonpath "$.count" matches "hi"
jsonpath "$.count" isEmpty
jsonpath "$.count" includes "foo"
jsonpath "$.count" isIsoDate
jsonpath "$.message" startsWith "hi"
jsonpath "$.message" endsWith "hi"
jsonpath "$.message" contains "hi"
jsonpath "$.message" matches "hi"
jsonpath "$.message" matches "hi{"
jsonpath "$.message" isEmpty
jsonpath "$.message" count == 1
jsonpath "$.message" isIsoDate
jsonpath "$.toto" exists
jsonpath "$.message" not exists
jsonpath "$.list" count == 2
jsonpath "$.not-exist" == 2
jsonpath "$.not-exist" > 3
jsonpath "$.not-exist" >= 3
jsonpath "$.not-exist" < 1
jsonpath "$.not-exist" <= 1
jsonpath "$.not-exist" count == 1
jsonpath "$.not-exist" startsWith "foo"
jsonpath "$.not-exist" endsWith "foo"
jsonpath "$.not-exist" contains "foo"
jsonpath "$.not-exist" includes "foo"
jsonpath "$.not-exist" matches /foo/
jsonpath "$.not-exist" isInteger
jsonpath "$.not-exist" isFloat
jsonpath "$.not-exist" isBoolean
jsonpath "$.not-exist" isString
jsonpath "$.not-exist" isCollection
jsonpath "$.not-exist" isDate
jsonpath "$.not-exist" exists
jsonpath "$.not-exist" isEmpty
jsonpath "$.not_a_date" isIsoDate
jsonpath "$.is_a_date" not isIsoDate
jsonpath "$.not_a_date" isNumber
